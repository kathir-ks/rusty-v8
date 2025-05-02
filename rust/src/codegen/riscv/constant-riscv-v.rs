// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod riscv_v {
    use crate::codegen::riscv::base_constants_riscv::*;

    // RVV Extension
    pub const OP_IVV: Opcode = OP_V | (0b000 << kFunct3Shift);
    pub const OP_FVV: Opcode = OP_V | (0b001 << kFunct3Shift);
    pub const OP_MVV: Opcode = OP_V | (0b010 << kFunct3Shift);
    pub const OP_IVI: Opcode = OP_V | (0b011 << kFunct3Shift);
    pub const OP_IVX: Opcode = OP_V | (0b100 << kFunct3Shift);
    pub const OP_FVF: Opcode = OP_V | (0b101 << kFunct3Shift);
    pub const OP_MVX: Opcode = OP_V | (0b110 << kFunct3Shift);

    pub const RO_V_VSETVLI: Opcode = OP_V | (0b111 << kFunct3Shift) | 0b0 << 31;
    pub const RO_V_VSETIVLI: Opcode = OP_V | (0b111 << kFunct3Shift) | 0b11 << 30;
    pub const RO_V_VSETVL: Opcode = OP_V | (0b111 << kFunct3Shift) | 0b1 << 31;

    // RVV LOAD/STORE
    pub const RO_V_VL: Opcode =
        LOAD_FP | (0b00 << kRvvMopShift) | (0b000 << kRvvNfShift);
    pub const RO_V_VLS: Opcode =
        LOAD_FP | (0b10 << kRvvMopShift) | (0b000 << kRvvNfShift);
    pub const RO_V_VLX: Opcode =
        LOAD_FP | (0b11 << kRvvMopShift) | (0b000 << kRvvNfShift);

    pub const RO_V_VS: Opcode =
        STORE_FP | (0b00 << kRvvMopShift) | (0b000 << kRvvNfShift);
    pub const RO_V_VSS: Opcode =
        STORE_FP | (0b10 << kRvvMopShift) | (0b000 << kRvvNfShift);
    pub const RO_V_VSX: Opcode =
        STORE_FP | (0b11 << kRvvMopShift) | (0b000 << kRvvNfShift);
    pub const RO_V_VSU: Opcode =
        STORE_FP | (0b01 << kRvvMopShift) | (0b000 << kRvvNfShift);
    // THE kFunct6Shift is mop
    pub const RO_V_VLSEG2: Opcode =
        LOAD_FP | (0b00 << kRvvMopShift) | (0b001 << kRvvNfShift);
    pub const RO_V_VLSEG3: Opcode =
        LOAD_FP | (0b00 << kRvvMopShift) | (0b010 << kRvvNfShift);
    pub const RO_V_VLSEG4: Opcode =
        LOAD_FP | (0b00 << kRvvMopShift) | (0b011 << kRvvNfShift);
    pub const RO_V_VLSEG5: Opcode =
        LOAD_FP | (0b00 << kRvvMopShift) | (0b100 << kRvvNfShift);
    pub const RO_V_VLSEG6: Opcode =
        LOAD_FP | (0b00 << kRvvMopShift) | (0b101 << kRvvNfShift);
    pub const RO_V_VLSEG7: Opcode =
        LOAD_FP | (0b00 << kRvvMopShift) | (0b110 << kRvvNfShift);
    pub const RO_V_VLSEG8: Opcode =
        LOAD_FP | (0b00 << kRvvMopShift) | (0b111 << kRvvNfShift);

    pub const RO_V_VSSEG2: Opcode =
        STORE_FP | (0b00 << kRvvMopShift) | (0b001 << kRvvNfShift);
    pub const RO_V_VSSEG3: Opcode =
        STORE_FP | (0b00 << kRvvMopShift) | (0b010 << kRvvNfShift);
    pub const RO_V_VSSEG4: Opcode =
        STORE_FP | (0b00 << kRvvMopShift) | (0b011 << kRvvNfShift);
    pub const RO_V_VSSEG5: Opcode =
        STORE_FP | (0b00 << kRvvMopShift) | (0b100 << kRvvNfShift);
    pub const RO_V_VSSEG6: Opcode =
        STORE_FP | (0b00 << kRvvMopShift) | (0b101 << kRvvNfShift);
    pub const RO_V_VSSEG7: Opcode =
        STORE_FP | (0b00 << kRvvMopShift) | (0b110 << kRvvNfShift);
    pub const RO_V_VSSEG8: Opcode =
        STORE_FP | (0b00 << kRvvMopShift) | (0b111 << kRvvNfShift);

    pub const RO_V_VLSSEG2: Opcode =
        LOAD_FP | (0b10 << kRvvMopShift) | (0b001 << kRvvNfShift);
    pub const RO_V_VLSSEG3: Opcode =
        LOAD_FP | (0b10 << kRvvMopShift) | (0b010 << kRvvNfShift);
    pub const RO_V_VLSSEG4: Opcode =
        LOAD_FP | (0b10 << kRvvMopShift) | (0b011 << kRvvNfShift);
    pub const RO_V_VLSSEG5: Opcode =
        LOAD_FP | (0b10 << kRvvMopShift) | (0b100 << kRvvNfShift);
    pub const RO_V_VLSSEG6: Opcode =
        LOAD_FP | (0b10 << kRvvMopShift) | (0b101 << kRvvNfShift);
    pub const RO_V_VLSSEG7: Opcode =
        LOAD_FP | (0b10 << kRvvMopShift) | (0b110 << kRvvNfShift);
    pub const RO_V_VLSSEG8: Opcode =
        LOAD_FP | (0b10 << kRvvMopShift) | (0b111 << kRvvNfShift);

    pub const RO_V_VSSSEG2: Opcode =
        STORE_FP | (0b10 << kRvvMopShift) | (0b001 << kRvvNfShift);
    pub const RO_V_VSSSEG3: Opcode =
        STORE_FP | (0b10 << kRvvMopShift) | (0b010 << kRvvNfShift);
    pub const RO_V_VSSSEG4: Opcode =
        STORE_FP | (0b10 << kRvvMopShift) | (0b011 << kRvvNfShift);
    pub const RO_V_VSSSEG5: Opcode =
        STORE_FP | (0b10 << kRvvMopShift) | (0b100 << kRvvNfShift);
    pub const RO_V_VSSSEG6: Opcode =
        STORE_FP | (0b10 << kRvvMopShift) | (0b101 << kRvvNfShift);
    pub const RO_V_VSSSEG7: Opcode =
        STORE_FP | (0b10 << kRvvMopShift) | (0b110 << kRvvNfShift);
    pub const RO_V_VSSSEG8: Opcode =
        STORE_FP | (0b10 << kRvvMopShift) | (0b111 << kRvvNfShift);

    pub const RO_V_VLXSEG2: Opcode =
        LOAD_FP | (0b11 << kRvvMopShift) | (0b001 << kRvvNfShift);
    pub const RO_V_VLXSEG3: Opcode =
        LOAD_FP | (0b11 << kRvvMopShift) | (0b010 << kRvvNfShift);
    pub const RO_V_VLXSEG4: Opcode =
        LOAD_FP | (0b11 << kRvvMopShift) | (0b011 << kRvvNfShift);
    pub const RO_V_VLXSEG5: Opcode =
        LOAD_FP | (0b11 << kRvvMopShift) | (0b100 << kRvvNfShift);
    pub const RO_V_VLXSEG6: Opcode =
        LOAD_FP | (0b11 << kRvvMopShift) | (0b101 << kRvvNfShift);
    pub const RO_V_VLXSEG7: Opcode =
        LOAD_FP | (0b11 << kRvvMopShift) | (0b110 << kRvvNfShift);
    pub const RO_V_VLXSEG8: Opcode =
        LOAD_FP | (0b11 << kRvvMopShift) | (0b111 << kRvvNfShift);

    pub const RO_V_VSXSEG2: Opcode =
        STORE_FP | (0b11 << kRvvMopShift) | (0b001 << kRvvNfShift);
    pub const RO_V_VSXSEG3: Opcode =
        STORE_FP | (0b11 << kRvvMopShift) | (0b010 << kRvvNfShift);
    pub const RO_V_VSXSEG4: Opcode =
        STORE_FP | (0b11 << kRvvMopShift) | (0b011 << kRvvNfShift);
    pub const RO_V_VSXSEG5: Opcode =
        STORE_FP | (0b11 << kRvvMopShift) | (0b100 << kRvvNfShift);
    pub const RO_V_VSXSEG6: Opcode =
        STORE_FP | (0b11 << kRvvMopShift) | (0b101 << kRvvNfShift);
    pub const RO_V_VSXSEG7: Opcode =
        STORE_FP | (0b11 << kRvvMopShift) | (0b110 << kRvvNfShift);
    pub const RO_V_VSXSEG8: Opcode =
        STORE_FP | (0b11 << kRvvMopShift) | (0b111 << kRvvNfShift);

    // RVV Vector Arithmetic Instruction
    pub const VADD_FUNCT6: Opcode = 0b000000;
    pub const RO_V_VADD_VI: Opcode = OP_IVI | (VADD_FUNCT6 << kRvvFunct6Shift);
    pub const RO_V_VADD_VV: Opcode = OP_IVV | (VADD_FUNCT6 << kRvvFunct6Shift);
    pub const RO_V_VADD_VX: Opcode = OP_IVX | (VADD_FUNCT6 << kRvvFunct6Shift);

    pub const VSUB_FUNCT6: Opcode = 0b000010;
    pub const RO_V_VSUB_VX: Opcode = OP_IVX | (VSUB_FUNCT6 << kRvvFunct6Shift);
    pub const RO_V_VSUB_VV: Opcode = OP_IVV | (VSUB_FUNCT6 << kRvvFunct6Shift);

    pub const VDIVU_FUNCT6: Opcode = 0b100000;
    pub const RO_V_VDIVU_VX: Opcode = OP_MVX | (VDIVU_FUNCT6 << kRvvFunct6Shift);
    pub const RO_V_VDIVU_VV: Opcode = OP_MVV | (VDIVU_FUNCT6 << kRvvFunct6Shift);

    pub const VDIV_FUNCT6: Opcode = 0b100001;
    pub const RO_V_VDIV_VX: Opcode = OP_MVX | (VDIV_FUNCT6 << kRvvFunct6Shift);
    pub const RO_V_VDIV_VV: Opcode = OP_MVV | (VDIV_FUNCT6 << kRvvFunct6Shift);

    pub const VREMU_FUNCT6: Opcode = 0b100010;
    pub const RO_V_VREMU_VX: Opcode = OP_MVX | (VREMU_FUNCT6 << kRvvFunct6Shift);
    pub const RO_V_VREMU_VV: Opcode = OP_MVV | (VREMU_FUNCT6 << kRvvFunct6Shift);

    pub const VREM_FUNCT6: Opcode = 0b100011;
    pub const RO_V_VREM_VX: Opcode = OP_MVX | (VREM_FUNCT6 << kRvvFunct6Shift);
    pub const RO_V_VREM_VV: Opcode = OP_MVV | (VREM_FUNCT6 << kRvvFunct6Shift);

    pub const VMULHU_FUNCT6: Opcode = 0b100100;
    pub const RO_V_VMULHU_VX: Opcode = OP_MVX | (VMULHU_FUNCT6 << kRvvFunct6Shift);
    pub const RO_V_VMULHU_VV: Opcode = OP_MVV | (VMULHU_FUNCT6 << kRvvFunct6Shift);

    pub const VMUL_FUNCT6: Opcode = 0b100101;
    pub const RO_V_VMUL_VX: Opcode = OP_MVX | (VMUL_FUNCT6 << kRvvFunct6Shift);
    pub const RO_V_VMUL_VV: Opcode = OP_MVV | (VMUL_FUNCT6 << kRvvFunct6Shift);

    pub const VWMUL_FUNCT6: Opcode = 0b111011;
    pub const RO_V_VWMUL_VX: Opcode = OP_MVX | (VWMUL_FUNCT6 << kRvvFunct6Shift);
    pub const RO_V_VWMUL_VV: Opcode = OP_MVV | (VWMUL_FUNCT6 << kRvvFunct6Shift);

    pub const VWMULU_FUNCT6: Opcode = 0b111000;
    pub const RO_V_VWMULU_VX: Opcode = OP_MVX | (VWMULU_FUNCT6 << kRvvFunct6Shift);
    pub const RO_V_VWMULU_VV: Opcode = OP_MVV | (VWMULU_FUNCT6 << kRvvFunct6Shift);

    pub const VMULHSU_FUNCT6: Opcode = 0b100110;
    pub const RO_V_VMULHSU_VX: Opcode = OP_MVX | (VMULHSU_FUNCT6 << kRvvFunct6Shift);
    pub const RO_V_VMULHSU_VV: Opcode = OP_MVV | (VMULHSU_FUNCT6 << kRvvFunct6Shift);

    pub const VMULH_FUNCT6: Opcode = 0b100111;
    pub const RO_V_VMULH_VX: Opcode = OP_MVX | (VMULH_FUNCT6 << kRvvFunct6Shift);
    pub const RO_V_VMULH_VV: Opcode = OP_MVV | (VMULH_FUNCT6 << kRvvFunct6Shift);

    pub const VWADD_FUNCT6: Opcode = 0b110001;
    pub const RO_V_VWADD_VV: Opcode = OP_MVV | (VWADD_FUNCT6 << kRvvFunct6Shift);
    pub const RO_V_VWADD_VX: Opcode = OP_MVX | (VWADD_FUNCT6 << kRvvFunct6Shift);

    pub const VWADDU_FUNCT6: Opcode = 0b110000;
    pub const RO_V_VWADDU_VV: Opcode = OP_MVV | (VWADDU_FUNCT6 << kRvvFunct6Shift);
    pub const RO_V_VWADDU_VX: Opcode = OP_MVX | (VWADDU_FUNCT6 << kRvvFunct6Shift);

    pub const VWADDUW_FUNCT6: Opcode = 0b110101;
    pub const RO_V_VWADDUW_VX: Opcode = OP_MVX | (VWADDUW_FUNCT6 << kRvvFunct6Shift);
    pub const RO_V_VWADDUW_VV: Opcode = OP_MVV | (VWADDUW_FUNCT6 << kRvvFunct6Shift);

    pub const VCOMPRESS_FUNCT6: Opcode = 0b010111;
    pub const RO_V_VCOMPRESS_VV: Opcode =
        OP_MVV | (VCOMPRESS_FUNCT6 << kRvvFunct6Shift);

    pub const VSADDU_FUNCT6: Opcode = 0b100000;
    pub const RO_V_VSADDU_VI: Opcode = OP_IVI | (VSADDU_FUNCT6 << kRvvFunct6Shift);
    pub const RO_V_VSADDU_VV: Opcode = OP_IVV | (VSADDU_FUNCT6 << kRvvFunct6Shift);
    pub const RO_V_VSADDU_VX: Opcode = OP_IVX | (VSADDU_FUNCT6 << kRvvFunct6Shift);

    pub const VSADD_FUNCT6: Opcode = 0b100001;
    pub const RO_V_VSADD_VI: Opcode = OP_IVI | (VSADD_FUNCT6 << kRvvFunct6Shift);
    pub const RO_V_VSADD_VV: Opcode = OP_IVV | (VSADD_FUNCT6 << kRvvFunct6Shift);
    pub const RO_V_VSADD_VX: Opcode = OP_IVX | (VSADD_FUNCT6 << kRvvFunct6Shift);

    pub const VSSUB_FUNCT6: Opcode = 0b100011;
    pub const RO_V_VSSUB_VV: Opcode = OP_IVV | (VSSUB_FUNCT6 << kRvvFunct6Shift);
    pub const RO_V_VSSUB_VX: Opcode = OP_IVX | (VSSUB_FUNCT6 << kRvvFunct6Shift);

    pub const VSSUBU_FUNCT6: Opcode = 0b100010;
    pub const RO_V_VSSUBU_VV: Opcode = OP_IVV | (VSSUBU_FUNCT6 << kRvvFunct6Shift);
    pub const RO_V_VSSUBU_VX: Opcode = OP_IVX | (VSSUBU_FUNCT6 << kRvvFunct6Shift);

    pub const VRSUB_FUNCT6: Opcode = 0b000011;
    pub const RO_V_VRSUB_VX: Opcode = OP_IVX | (VRSUB_FUNCT6 << kRvvFunct6Shift);
    pub const RO_V_VRSUB_VI: Opcode = OP_IVI | (VRSUB_FUNCT6 << kRvvFunct6Shift);

    pub const VMINU_FUNCT6: Opcode = 0b000100;
    pub const RO_V_VMINU_VX: Opcode = OP_IVX | (VMINU_FUNCT6 << kRvvFunct6Shift);
    pub const RO_V_VMINU_VV: Opcode = OP_IVV | (VMINU_FUNCT6 << kRvvFunct6Shift);

    pub const VMIN_FUNCT6: Opcode = 0b000101;
    pub const RO_V_VMIN_VX: Opcode = OP_IVX | (VMIN_FUNCT6 << kRvvFunct6Shift);
    pub const RO_V_VMIN_VV: Opcode = OP_IVV | (VMIN_FUNCT6 << kRvvFunct6Shift);

    pub const VMAXU_FUNCT6: Opcode = 0b000110;
    pub const RO_V_VMAXU_VX: Opcode = OP_IVX | (VMAXU_FUNCT6 << kRvvFunct6Shift);
    pub const RO_V_VMAXU_VV: Opcode = OP_IVV | (VMAXU_FUNCT6 << kRvvFunct6Shift);

    pub const VMAX_FUNCT6: Opcode = 0b000111;
    pub const RO_V_VMAX_VX: Opcode = OP_IVX | (VMAX_FUNCT6 << kRvvFunct6Shift);
    pub const RO_V_VMAX_VV: Opcode = OP_IVV | (VMAX_FUNCT6 << kRvvFunct6Shift);

    pub const VAND_FUNCT6: Opcode = 0b001001;
    pub const RO_V_VAND_VI: Opcode = OP_IVI | (VAND_FUNCT6 << kRvvFunct6Shift);
    pub const RO_V_VAND_VV: Opcode = OP_IVV | (VAND_FUNCT6 << kRvvFunct6Shift);
    pub const RO_V_VAND_VX: Opcode = OP_IVX | (VAND_FUNCT6 << kRvvFunct6Shift);

    pub const VOR_FUNCT6: Opcode = 0b001010;
    pub const RO_V_VOR_VI: Opcode = OP_IVI | (VOR_FUNCT6 << kRvvFunct6Shift);
    pub const RO_V_VOR_VV: Opcode = OP_IVV | (VOR_FUNCT6 << kRvvFunct6Shift);
    pub const RO_V_VOR_VX: Opcode = OP_IVX | (VOR_FUNCT6 << kRvvFunct6Shift);

    pub const VXOR_FUNCT6: Opcode = 0b001011;
    pub const RO_V_VXOR_VI: Opcode = OP_IVI | (VXOR_FUNCT6 << kRvvFunct6Shift);
    pub const RO_V_VXOR_VV: Opcode = OP_IVV | (VXOR_FUNCT6 << kRvvFunct6Shift);
    pub const RO_V_VXOR_VX: Opcode = OP_IVX | (VXOR_FUNCT6 << kRvvFunct6Shift);

    pub const VRGATHER_FUNCT6: Opcode = 0b001100;
    pub const RO_V_VRGATHER_VI: Opcode =
        OP_IVI | (VRGATHER_FUNCT6 << kRvvFunct6Shift);
    pub const RO_V_VRGATHER_VV: Opcode =
        OP_IVV | (VRGATHER_FUNCT6 << kRvvFunct6Shift);
    pub const RO_V_VRGATHER_VX: Opcode =
        OP_IVX | (VRGATHER_FUNCT6 << kRvvFunct6Shift);

    pub const VMV_FUNCT6: Opcode = 0b010111;
    pub const RO_V_VMV_VI: Opcode = OP_IVI | (VMV_FUNCT6 << kRvvFunct6Shift);
    pub const RO_V_VMV_VV: Opcode = OP_IVV | (VMV_FUNCT6 << kRvvFunct6Shift);
    pub const RO_V_VMV_VX: Opcode = OP_IVX | (VMV_FUNCT6 << kRvvFunct6Shift);
    pub const RO_V_VFMV_VF: Opcode = OP_FVF | (VMV_FUNCT6 << kRvvFunct6Shift);

    pub const RO_V_VMERGE_VI: Opcode = RO_V_VMV_VI;
    pub const RO_V_VMERGE_VV: Opcode = RO_V_VMV_VV;
    pub const RO_V_VMERGE_VX: Opcode = RO_V_VMV_VX;
    pub const RO_V_VFMERGE_VF: Opcode = RO_V_VFMV_VF;

    pub const VMSEQ_FUNCT6: Opcode = 0b011000;
    pub const RO_V_VMSEQ_VI: Opcode = OP_IVI | (VMSEQ_FUNCT6 << kRvvFunct6Shift);
    pub const RO_V_VMSEQ_VV: Opcode = OP_IVV | (VMSEQ_FUNCT6 << kRvvFunct6Shift);
    pub const RO_V_VMSEQ_VX: Opcode = OP_IVX | (VMSEQ_FUNCT6 << kRvvFunct6Shift);

    pub const VMSNE_FUNCT6: Opcode = 0b011001;
    pub const RO_V_VMSNE_VI: Opcode = OP_IVI | (VMSNE_FUNCT6 << kRvvFunct6Shift);
    pub const RO_V_VMSNE_VV: Opcode = OP_IVV | (VMSNE_FUNCT6 << kRvvFunct6Shift);
    pub const RO_V_VMSNE_VX: Opcode = OP_IVX | (VMSNE_FUNCT6 << kRvvFunct6Shift);

    pub const VMSLTU_FUNCT6: Opcode = 0b011010;
    pub const RO_V_VMSLTU_VV: Opcode = OP_IVV | (VMSLTU_FUNCT6 << kRvvFunct6Shift);
    pub const RO_V_VMSLTU_VX: Opcode = OP_IVX | (VMSLTU_FUNCT6 << kRvvFunct6Shift);

    pub const VMSLT_FUNCT6: Opcode = 0b011011;
    pub const RO_V_VMSLT_VV: Opcode = OP_IVV | (VMSLT_FUNCT6 << kRvvFunct6Shift);
    pub const RO_V_VMSLT_VX: Opcode = OP_IVX | (VMSLT_FUNCT6 << kRvvFunct6Shift);

    pub const VMSLE_FUNCT6: Opcode = 0b011101;
    pub const RO_V_VMSLE_VI: Opcode = OP_IVI | (VMSLE_FUNCT6 << kRvvFunct6Shift);
    pub const RO_V_VMSLE_VV: Opcode = OP_IVV | (VMSLE_FUNCT6 << kRvvFunct6Shift);
    pub const RO_V_VMSLE_VX: Opcode = OP_IVX | (VMSLE_FUNCT6 << kRvvFunct6Shift);

    pub const VMSLEU_FUNCT6: Opcode = 0b011100;
    pub const RO_V_VMSLEU_VI: Opcode = OP_IVI | (VMSLEU_FUNCT6 << kRvvFunct6Shift);
    pub const RO_V_VMSLEU_VV: Opcode = OP_IVV | (VMSLEU_FUNCT6 << kRvvFunct6Shift);
    pub const RO_V_VMSLEU_VX: Opcode = OP_IVX | (VMSLEU_FUNCT6 << kRvvFunct6Shift);

    pub const VMSGTU_FUNCT6: Opcode = 0b011110;
    pub const RO_V_VMSGTU_VI: Opcode = OP_IVI | (VMSGTU_FUNCT6 << kRvvFunct6Shift);
    pub const RO_V_VMSGTU_VX: Opcode = OP_IVX | (VMSGTU_FUNCT6 << kRvvFunct6Shift);

    pub const VMSGT_FUNCT6: Opcode = 0b011111;
    pub const RO_V_VMSGT_VI: Opcode = OP_IVI | (VMSGT_FUNCT6 << kRvvFunct6Shift);
    pub const RO_V_VMSGT_VX: Opcode = OP_IVX | (VMSGT_FUNCT6 << kRvvFunct6Shift);

    pub const VSLIDEUP_FUNCT6: Opcode = 0b001110;
    pub const RO_V_VSLIDEUP_VI: Opcode =
        OP_IVI | (VSLIDEUP_FUNCT6 << kRvvFunct6Shift);
    pub const RO_V_VSLIDEUP_VX: Opcode =
        OP_IVX | (VSLIDEUP_FUNCT6 << kRvvFunct6Shift);
    pub const RO_V_VSLIDE1UP_VX: Opcode =
        OP_MVX | (VSLIDEUP_FUNCT6 << kRvvFunct6Shift);
    pub const RO_V_VFSLIDE1UP_VF: Opcode =
        OP_FVF | (VSLIDEUP_FUNCT6 << kRvvFunct6Shift);

    pub const VSLIDEDOWN_FUNCT6: Opcode = 0b001111;
    pub const RO_V_VSLIDEDOWN_VI: Opcode =
        OP_IVI | (VSLIDEDOWN_FUNCT6 << kRvvFunct6Shift);
    pub const RO_V_VSLIDEDOWN_VX: Opcode =
        OP_IVX | (VSLIDEDOWN_FUNCT6 << kRvvFunct6Shift);
    pub const RO_V_VSLIDE1DOWN_VX: Opcode =
        OP_MVX | (VSLIDEDOWN_FUNCT6 << kRvvFunct6Shift);
    pub const RO_V_VFSLIDE1DOWN_VF: Opcode =
        OP_FVF | (VSLIDEDOWN_FUNCT6 << kRvvFunct6Shift);

    pub const VSRL_FUNCT6: Opcode = 0b101000;
    pub const RO_V_VSRL_VI: Opcode = OP_IVI | (VSRL_FUNCT6 << kRvvFunct6Shift);
    pub const RO_V_VSRL_VV: Opcode = OP_IVV | (VSRL_FUNCT6 << kRvvFunct6Shift);
    pub const RO_V_VSRL_VX: Opcode = OP_IVX | (VSRL_FUNCT6 << kRvvFunct6Shift);

    pub const VSRA_FUNCT6: Opcode = 0b101001;
    pub const RO_V_VSRA_VI: Opcode = OP_IVI | (VSRA_FUNCT6 << kRvvFunct6Shift);
    pub const RO_V_VSRA_VV: Opcode = OP_IVV | (VSRA_FUNCT6 << kRvvFunct6Shift);
    pub const RO_V_VSRA_VX: Opcode = OP_IVX | (VSRA_FUNCT6 << kRvvFunct6Shift);

    pub const VSLL_FUNCT6: Opcode = 0b100101;
    pub const RO_V_VSLL_VI: Opcode = OP_IVI | (VSLL_FUNCT6 << kRvvFunct6Shift);
    pub const RO_V_VSLL_VV: Opcode = OP_IVV | (VSLL_FUNCT6 << kRvvFunct6Shift);
    pub const RO_V_VSLL_VX: Opcode = OP_IVX | (VSLL_FUNCT6 << kRvvFunct6Shift);

    pub const VSMUL_FUNCT6: Opcode = 0b100111;
    pub const RO_V_VSMUL_VV: Opcode = OP_IVV | (VSMUL_FUNCT6 << kRvvFunct6Shift);
    pub const RO_V_VSMUL_VX: Opcode = OP_IVX | (VSMUL_FUNCT6 << kRvvFunct6Shift);

    pub