"""Unit tests for the layered validation harness.

Fast tests (no transpile/build) covering the spec scenarios for taxonomy,
module mapping, manifest rejection, normalization, and scorecard/regression.
"""

import pytest

from cpp2rust.validation.taxonomy import load_taxonomy, TaxonomyError, Taxonomy, Layer
from cpp2rust.validation.module_map import load_module_map, UNASSIGNED
from cpp2rust.validation.manifest import discover_manifests, load_manifest, ManifestError
from cpp2rust.validation.normalize import normalize, available_rules
from cpp2rust.validation.report import (
    build_scorecard, diff_baseline, Scorecard, ROW_PASS, ROW_UNIMPLEMENTED,
)
from cpp2rust.validation.harness import LayerResult, LAYER_PASS, LAYER_NOT_RUN
from cpp2rust.validation import LAYERS_ROOT


class TestTaxonomy:
    def test_loads_and_is_valid(self):
        t = load_taxonomy()
        assert 5 <= len(t.layers) <= 10
        # contiguous ordering
        assert [l.order for l in t.ordered()] == list(range(1, len(t.layers) + 1))

    def test_required_stages_present(self):
        ids = " ".join(load_taxonomy().ids())
        for stage in ("scanner", "parser", "ast", "bytecode"):
            assert stage in ids

    def test_rejects_too_few_layers(self, tmp_path):
        p = tmp_path / "tax.yaml"
        p.write_text("layers:\n" + "".join(
            f"  - id: l{i}\n    order: {i}\n    description: d\n" for i in range(1, 4)))
        with pytest.raises(TaxonomyError):
            load_taxonomy(p)

    def test_rejects_duplicate_order(self, tmp_path):
        p = tmp_path / "tax.yaml"
        body = "".join(
            f"  - id: scanner{i}\n    order: 1\n    description: parser ast bytecode\n"
            for i in range(5))
        p.write_text("layers:\n" + body)
        with pytest.raises(TaxonomyError):
            load_taxonomy(p)


class TestModuleMap:
    def test_longest_prefix_wins(self):
        mm = load_module_map()
        assert mm.layer_for("codebase/src/parsing/scanner.cc") == "scanner"
        assert mm.layer_for("src/parsing/parser.cc") == "parser"
        assert mm.layer_for("src/parsing/whatever.cc") == "source-intake"

    def test_unmapped_is_explicit(self):
        mm = load_module_map()
        assert mm.layer_for("src/wasm/foo.cc") == UNASSIGNED
        assert not mm.is_mapped("src/wasm/foo.cc")


class TestManifest:
    def test_real_manifests_load(self):
        for layer_id, path in discover_manifests(LAYERS_ROOT).items():
            m = load_manifest(path)
            assert m.has_source_set
            assert m.output_format
            assert m.fixtures_dir.exists()

    def test_rejects_missing_output_format(self, tmp_path):
        d = tmp_path / "x"
        (d / "fixtures").mkdir(parents=True)
        mp = d / "manifest.yaml"
        mp.write_text("layer: x\nsource_set: [a.cc]\nfixtures_dir: fixtures\n")
        with pytest.raises(ManifestError):
            load_manifest(mp)

    def test_rejects_missing_source_set(self, tmp_path):
        d = tmp_path / "x"
        (d / "fixtures").mkdir(parents=True)
        mp = d / "manifest.yaml"
        mp.write_text("layer: x\noutput_format: token-stream\nfixtures_dir: fixtures\n")
        with pytest.raises(ManifestError):
            load_manifest(mp)


class TestNormalize:
    def test_strip_addresses_and_ids(self):
        out = normalize("Add a0 0xdeadbeef #42", ["strip-addresses", "strip-ids"])
        assert "0x" not in out and "#42" not in out
        assert "<addr>" in out and "#<id>" in out

    def test_unknown_rule_raises(self):
        with pytest.raises(KeyError):
            normalize("x", ["no-such-rule"])

    def test_baseline_strips_trailing_ws(self):
        assert normalize("a   \nb\t", []) == "a\nb"

    def test_rules_listed(self):
        assert "strip-addresses" in available_rules()


def _result(layer, verdict, fx_run=0, fx_pass=0):
    r = LayerResult(layer=layer, verdict=verdict)
    return r, fx_run, fx_pass


class TestScorecard:
    def _taxonomy(self):
        return load_taxonomy()

    def test_covers_every_layer_including_unimplemented(self):
        t = self._taxonomy()
        card = build_scorecard(t, {})  # nothing run
        assert len(card.rows) == len(t.layers)
        assert all(r.status == ROW_UNIMPLEMENTED for r in card.rows)

    def test_aggregate_counts_passing(self):
        t = self._taxonomy()
        first = t.ordered()[0].id
        res = LayerResult(layer=first, verdict=LAYER_PASS)
        res.fixtures = []  # passing with stage-level only
        card = build_scorecard(t, {first: res})
        assert card.aggregate.layers_total == len(t.layers)
        assert card.aggregate.layers_passing == 1

    def test_regression_distinguished_from_never_passed(self):
        t = self._taxonomy()
        first = t.ordered()[0].id
        # baseline: first layer was passing
        baseline = {"rows": [{"layer": first, "status": ROW_PASS}]}
        # now it is not-run -> regression
        res = LayerResult(layer=first, verdict=LAYER_NOT_RUN)
        card = build_scorecard(t, {first: res})
        rep = diff_baseline(card, baseline)
        assert first in rep.regressions
        assert rep.has_regressions
        # a layer that never passed is outstanding, not a regression
        assert all(l not in rep.regressions for l in t.ids() if l != first)
