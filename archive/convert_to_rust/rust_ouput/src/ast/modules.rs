// Converted from V8 C++ source files:
// Header: modules.h
// Implementation: modules.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod modules {
    use std::collections::{HashMap, HashSet};
    use std::hash::{Hash, Hasher};
    use std::rc::Rc;

    use crate::ast::ast::ModuleImportPhase;
    use crate::parsing::import_attributes::ImportAttributes;
    use crate::parsing::scanner::Location;
    use crate::strings::string_stream::FixedArray;

    pub struct ZoneObject {}

    pub struct SourceTextModuleDescriptor {
        module_requests: ModuleRequestMap,
        special_exports: Vec<Rc<Entry>>,
        namespace_imports: Vec<Rc<Entry>>,
        regular_exports: RegularExportMap,
        regular_imports: RegularImportMap,
    }

    impl SourceTextModuleDescriptor {
        pub fn new() -> Self {
            SourceTextModuleDescriptor {
                module_requests: ModuleRequestMap::new(),
                special_exports: Vec::new(),
                namespace_imports: Vec::new(),
                regular_exports: RegularExportMap::new(),
                regular_imports: RegularImportMap::new(),
            }
        }

        pub fn add_import(
            &mut self,
            import_name: Rc<AstRawString>,
            local_name: Rc<AstRawString>,
            specifier: Rc<AstRawString>,
            import_phase: ModuleImportPhase,
            import_attributes: Rc<ImportAttributes>,
            loc: Location,
            specifier_loc: Location,
        ) {
            let module_request = self.add_module_request(
                specifier.clone(),
                import_phase,
                import_attributes,
                specifier_loc,
            );
            let entry = Rc::new(Entry::new(loc, Some(local_name), Some(import_name), None, module_request, 0));
            self.add_regular_import(entry);
        }

        pub fn add_star_import(
            &mut self,
            local_name: Rc<AstRawString>,
            specifier: Rc<AstRawString>,
            import_attributes: Rc<ImportAttributes>,
            loc: Location,
            specifier_loc: Location,
        ) {
            let module_request = self.add_module_request(
                specifier.clone(),
                ModuleImportPhase::kEvaluation,
                import_attributes,
                specifier_loc,
            );
            let entry = Rc::new(Entry::new(loc, Some(local_name), None, None, module_request, 0));
            self.add_namespace_import(entry);
        }

        pub fn add_empty_import(
            &mut self,
            specifier: Rc<AstRawString>,
            import_attributes: Rc<ImportAttributes>,
            specifier_loc: Location,
        ) {
            self.add_module_request(
                specifier,
                ModuleImportPhase::kEvaluation,
                import_attributes,
                specifier_loc,
            );
        }

        pub fn add_export(
            &mut self,
            local_name: Rc<AstRawString>,
            export_name: Rc<AstRawString>,
            loc: Location,
        ) {
            let entry = Rc::new(Entry::new(loc, Some(local_name), None, Some(export_name), -1, 0));
            self.add_regular_export(entry);
        }

        pub fn add_export_from(
            &mut self,
            import_name: Rc<AstRawString>,
            export_name: Rc<AstRawString>,
            specifier: Rc<AstRawString>,
            import_attributes: Rc<ImportAttributes>,
            loc: Location,
            specifier_loc: Location,
        ) {
            let module_request = self.add_module_request(
                specifier.clone(),
                ModuleImportPhase::kEvaluation,
                import_attributes,
                specifier_loc,
            );
            let entry = Rc::new(Entry::new(loc, None, Some(import_name), Some(export_name), module_request, 0));
            self.add_special_export(entry);
        }

        pub fn add_star_export(
            &mut self,
            specifier: Rc<AstRawString>,
            import_attributes: Rc<ImportAttributes>,
            loc: Location,
            specifier_loc: Location,
        ) {
            let module_request = self.add_module_request(
                specifier.clone(),
                ModuleImportPhase::kEvaluation,
                import_attributes,
                specifier_loc,
            );
            let entry = Rc::new(Entry::new(loc, None, None, None, module_request, 0));
            self.add_special_export(entry);
        }

        pub fn validate(
            &mut self,
            module_scope: &mut ModuleScope,
            error_handler: &mut dyn PendingCompilationErrorHandler,
        ) -> bool {
            if let Some(entry) = self.find_duplicate_export() {
                error_handler.report_message_at(
                    entry.location.beg_pos,
                    entry.location.end_pos,
                    MessageTemplate::kDuplicateExport,
                    entry.export_name.as_ref().unwrap().clone(),
                );
                return false;
            }

            for (local_name, entries) in self.regular_exports.iter() {
                for entry in entries {
                    if module_scope.lookup_local(entry.local_name().as_ref().unwrap().clone()).is_none() {
                        error_handler.report_message_at(
                            entry.location().beg_pos,
                            entry.location().end_pos,
                            MessageTemplate::kModuleExportUndefined,
                            entry.local_name().as_ref().unwrap().clone(),
                        );
                        return false;
                    }
                }
            }

            self.make_indirect_exports_explicit();
            self.assign_cell_indices();
            true
        }

        pub fn module_requests(&self) -> &ModuleRequestMap {
            &self.module_requests
        }

        pub fn namespace_imports(&self) -> &Vec<Rc<Entry>> {
            &self.namespace_imports
        }

        pub fn regular_imports(&self) -> &RegularImportMap {
            &self.regular_imports
        }

        pub fn special_exports(&self) -> &Vec<Rc<Entry>> {
            &self.special_exports
        }

        pub fn regular_exports(&self) -> &RegularExportMap {
            &self.regular_exports
        }

        fn add_regular_export(&mut self, entry: Rc<Entry>) {
            let local_name = entry.local_name().clone().unwrap();
            self.regular_exports.entry(local_name).or_insert_vec().push(entry);
        }

        fn add_special_export(&mut self, entry: Rc<Entry>) {
            self.special_exports.push(entry);
        }

        fn add_regular_import(&mut self, entry: Rc<Entry>) {
            let local_name = entry.local_name().clone().unwrap();
            self.regular_imports.insert(local_name, entry);
        }

        fn add_namespace_import(&mut self, entry: Rc<Entry>) {
            self.namespace_imports.push(entry);
        }

        fn find_duplicate_export(&self) -> Option<Rc<Entry>> {
            let mut export_names: HashMap<Rc<AstRawString>, Rc<Entry>> = HashMap::new();
            let mut duplicate: Option<Rc<Entry>> = None;

            for (_, entries) in &self.regular_exports {
                for entry in entries {
                    duplicate = Self::better_duplicate(entry.clone(), &mut export_names, duplicate);
                }
            }

            for entry in &self.special_exports {
                if entry.export_name().is_none() {
                    continue;
                }
                duplicate = Self::better_duplicate(entry.clone(), &mut export_names, duplicate);
            }

            duplicate
        }

        fn better_duplicate(
            candidate: Rc<Entry>,
            export_names: &mut HashMap<Rc<AstRawString>, Rc<Entry>>,
            current_duplicate: Option<Rc<Entry>>,
        ) -> Option<Rc<Entry>> {
            let export_name = candidate.export_name().clone().unwrap();
            if export_names.contains_key(&export_name) {
                if current_duplicate.is_none() {
                    let existing_entry = export_names.get(&export_name).unwrap().clone();
                    Some(if candidate.location().beg_pos > existing_entry.location().beg_pos {
                        candidate
                    } else {
                        existing_entry
                    })
                } else {
                    let current = current_duplicate.unwrap();
                    Some(if candidate.location().beg_pos > current.location().beg_pos {
                        candidate
                    } else {
                        current
                    })
                }
            } else {
                export_names.insert(export_name, candidate.clone());
                current_duplicate
            }
        }

        fn make_indirect_exports_explicit(&mut self) {
            let mut indirect_exports: Vec<(Rc<AstRawString>, Rc<Entry>)> = Vec::new();

            for (local_name, entries) in self.regular_exports.iter() {
                for entry in entries {
                    if let Some(import) = self.regular_imports.get(local_name) {
                        if entry.import_name().is_none() && entry.module_request() < 0 {
                            if import.import_name().is_some() && import.module_request() >= 0 {
                                let mut mutable_entry = entry.clone();
                                mutable_entry.set_import_name(import.import_name().clone());
                                mutable_entry.set_module_request(import.module_request());
                                mutable_entry.set_location(import.location());
                                mutable_entry.set_local_name(None);

                                indirect_exports.push((local_name.clone(), mutable_entry));
                            }
                        }
                    }
                }
            }

            for (local_name, mutable_entry) in indirect_exports {
                self.add_special_export(mutable_entry.clone());

                if let Some(entries) = self.regular_exports.get_mut(&local_name) {
                    entries.retain(|entry| !Rc::ptr_eq(entry, &mutable_entry));
                }
                self.regular_exports.retain(|_, entries| !entries.is_empty());
            }
        }

        fn assign_cell_indices(&mut self) {
            let mut export_index: i32 = 1;
            for (_, entries) in self.regular_exports.iter_mut() {
                for entry in entries {
                    entry.set_cell_index(export_index);
                }
                export_index += 1;
            }

            let mut import_index: i32 = -1;
            for (_, entry) in self.regular_imports.iter_mut() {
                entry.set_cell_index(import_index);
                import_index -= 1;
            }
        }

        fn add_module_request(
            &mut self,
            specifier: Rc<AstRawString>,
            import_phase: ModuleImportPhase,
            import_attributes: Rc<ImportAttributes>,
            specifier_loc: Location,
        ) -> i32 {
            let module_requests_count = self.module_requests.len() as i32;

            let ast_module_request = AstModuleRequest::new(
                specifier,
                import_phase,
                import_attributes,
                specifier_loc.beg_pos,
                module_requests_count,
            );

            if self.module_requests.contains_key(&ast_module_request) {
                return self.module_requests.get(&ast_module_request).unwrap().index();
            }

            let index = ast_module_request.index();
            self.module_requests.insert(ast_module_request, ast_module_request);
            index
        }

        pub fn serialize_regular_exports(&self) -> Result<FixedArray, String> {
            let regular_exports_size = self.regular_exports.len();
            let mut data: Vec<String> = Vec::with_capacity(
                SourceTextModuleInfo::kRegularExportLength * regular_exports_size,
            );

            for (local_name, entries) in self.regular_exports.iter() {
                let count = entries.len();
                let mut export_names: Vec<String> = Vec::with_capacity(count);
                for entry in entries.iter() {
                    export_names.push(entry.export_name().clone().unwrap().string());
                }

                data.push(local_name.string());
                data.push(entries[0].cell_index().to_string());
                data.push(export_names.join(","));
            }

            let result = FixedArray {
                array: data.join("").into_bytes(),
            };

            Ok(result)
        }

        pub fn get_cell_index_kind(cell_index: i32) -> CellIndexKind {
            if cell_index > 0 {
                CellIndexKind::kExport
            } else if cell_index < 0 {
                CellIndexKind::kImport
            } else {
                CellIndexKind::kInvalid
            }
        }
    }

    pub struct Entry {
        location: Location,
        export_name: Option<Rc<AstRawString>>,
        local_name: Option<Rc<AstRawString>>,
        import_name: Option<Rc<AstRawString>>,
        module_request: i32,
        cell_index: i32,
    }

    impl Entry {
        pub fn new(
            location: Location,
            local_name: Option<Rc<AstRawString>>,
            import_name: Option<Rc<AstRawString>>,
            export_name: Option<Rc<AstRawString>>,
            module_request: i32,
            cell_index: i32,
        ) -> Self {
            Entry {
                location,
                export_name,
                local_name,
                import_name,
                module_request,
                cell_index,
            }
        }

        pub fn location(&self) -> Location {
            self.location
        }

        pub fn export_name(&self) -> &Option<Rc<AstRawString>> {
            &self.export_name
        }

        pub fn local_name(&self) -> &Option<Rc<AstRawString>> {
            &self.local_name
        }

        pub fn import_name(&self) -> &Option<Rc<AstRawString>> {
            &self.import_name
        }

        pub fn module_request(&self) -> i32 {
            self.module_request
        }

        pub fn cell_index(&self) -> i32 {
            self.cell_index
        }

        pub fn set_location(&mut self, location: Location) {
            self.location = location;
        }

        pub fn set_export_name(&mut self, export_name: Option<Rc<AstRawString>>) {
            self.export_name = export_name;
        }

        pub fn set_local_name(&mut self, local_name: Option<Rc<AstRawString>>) {
            self.local_name = local_name;
        }

        pub fn set_import_name(&mut self, import_name: Option<Rc<AstRawString>>) {
            self.import_name = import_name;
        }

        pub fn set_module_request(&mut self, module_request: i32) {
            self.module_request = module_request;
        }

        pub fn set_cell_index(&mut self, cell_index: i32) {
            self.cell_index = cell_index;
        }
    }

    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AstModuleRequest {
        specifier: Rc<AstRawString>,
        phase: ModuleImportPhase,
        import_attributes: Rc<ImportAttributes>,
        position: i32,
        index: i32,
    }

    impl AstModuleRequest {
        pub fn new(
            specifier: Rc<AstRawString>,
            phase: ModuleImportPhase,
            import_attributes: Rc<ImportAttributes>,
            position: i32,
            index: i32,
        ) -> Self {
            AstModuleRequest {
                specifier,
                phase,
                import_attributes,
                position,
                index,
            }
        }

        pub fn specifier(&self) -> &Rc<AstRawString> {
            &self.specifier
        }
        pub fn import_attributes(&self) -> &Rc<ImportAttributes> {
            &self.import_attributes
        }
        pub fn phase(&self) -> ModuleImportPhase {
            self.phase
        }
        pub fn position(&self) -> i32 {
            self.position
        }
        pub fn index(&self) -> i32 {
            self.index
        }
    }

    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    pub struct AstRawString {
        string: String,
    }

    impl AstRawString {
        pub fn new(string: String) -> Self {
            AstRawString { string }
        }

        pub fn compare(lhs: &AstRawString, rhs: &AstRawString) -> i32 {
            lhs.string.cmp(&rhs.string) as i32
        }

        pub fn string(&self) -> String {
            self.string.clone()
        }
    }

    pub struct AstRawStringComparer {}

    impl AstRawStringComparer {
        pub fn compare(lhs: &Rc<AstRawString>, rhs: &Rc<AstRawString>) -> bool {
            AstRawString::compare(&lhs, &rhs) < 0
        }
    }

    pub struct ModuleRequestComparer {}

    impl ModuleRequestComparer {
        pub fn compare(lhs: &Rc<AstModuleRequest>, rhs: &Rc<AstModuleRequest>) -> bool {
            if AstRawString::compare(&lhs.specifier, &rhs.specifier) != 0 {
                return AstRawString::compare(&lhs.specifier, &rhs.specifier) < 0;
            }

            if lhs.phase != rhs.phase {
                return lhs.phase < rhs.phase;
            }

            false
        }
    }

    pub type ModuleRequestMap = HashMap<AstModuleRequest, AstModuleRequest>;
    pub type RegularExportMap = HashMap<Rc<AstRawString>, Vec<Rc<Entry>>>;
    pub type RegularImportMap = HashMap<Rc<AstRawString>, Rc<Entry>>;

    pub enum CellIndexKind {
        kInvalid,
        kExport,
        kImport,
    }

    pub struct ModuleScope {}

    impl ModuleScope {
        pub fn lookup_local(&self, _name: Rc<AstRawString>) -> Option<()> {
            Some(())
        }

        pub fn module(&self) -> &SourceTextModuleDescriptor {
            &SourceTextModuleDescriptor::new()
        }
    }

    pub trait PendingCompilationErrorHandler {
        fn report_message_at(
            &mut self,
            beg_pos: i32,
            end_pos: i32,
            message_template: MessageTemplate,
            arg: Rc<AstRawString>,
        );
    }

    pub enum MessageTemplate {
        kDuplicateExport,
        kModuleExportUndefined,
    }

    pub struct SourceTextModuleInfo {}

    impl SourceTextModuleInfo {
        pub const kRegularExportLength: usize = 3;
        pub const kRegularExportLocalNameOffset: usize = 0;
        pub const kRegularExportCellIndexOffset: usize = 1;
        pub const kRegularExportExportNamesOffset: usize = 2;
    }
}
