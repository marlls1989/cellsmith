primitive DFF_Q(Q, D, CLK); // clock CLK is the last port
output Q;
input  D, CLK;
reg    Q;
table
	(??) ? : ? : -;
	0 (01) : ? : 0;
	1 (01) : ? : 1;
	? (10) : ? : -;
endtable
endprimitive
`celldefine
module DFF(Q, CLK, D);
output Q;
input  CLK, D;
specify
	(CLK => Q) = (0.1, 0.1);
	(D => Q) = (0.1, 0.1);
endspecify
DFF_Q u_DFF_Q (Q, D, CLK);
endmodule
`endcelldefine
primitive DFF_NOCOLLAPSE_Q(Q, CLK, M);
output Q;
input  CLK, M;
reg    Q;
table
	0 ? : ? : -;
	1 0 : ? : 0;
	1 1 : ? : 1;
endtable
endprimitive
primitive DFF_NOCOLLAPSE_M(M, CLK, D);
output M;
input  CLK, D;
reg    M;
table
	0 0 : ? : 0;
	0 1 : ? : 1;
	1 ? : ? : -;
endtable
endprimitive
`celldefine
module DFF_NOCOLLAPSE(Q, CLK, D);
output Q;
input  CLK, D;
wire   M;
specify
	(CLK => Q) = (0.1, 0.1);
	(D => Q) = (0.1, 0.1);
endspecify
DFF_NOCOLLAPSE_Q u_DFF_NOCOLLAPSE_Q (Q, CLK, M);
DFF_NOCOLLAPSE_M u_DFF_NOCOLLAPSE_M (M, CLK, D);
endmodule
`endcelldefine
primitive UCDFF_Q(Q, CLK, M);
output Q;
input  CLK, M;
reg    Q;
table
	0 ? : ? : -;
	1 0 : ? : 0;
	1 1 : ? : 1;
endtable
endprimitive
primitive UCDFF_M(M, CLK, D);
output M;
input  CLK, D;
reg    M;
table
	0 0 : ? : 0;
	0 1 : ? : 1;
	1 ? : ? : -;
endtable
endprimitive
`celldefine
module UCDFF(Q, CLK, D);
output Q;
input  CLK, D;
wire   M;
specify
	(CLK => Q) = (0.1, 0.1);
	(D => Q) = (0.1, 0.1);
endspecify
UCDFF_Q u_UCDFF_Q (Q, CLK, M);
UCDFF_M u_UCDFF_M (M, CLK, D);
endmodule
`endcelldefine
primitive EMDFF_M(M, CLK, D);
output M;
input  CLK, D;
reg    M;
table
	0 0 : ? : 0;
	0 1 : ? : 1;
	1 ? : ? : -;
endtable
endprimitive
primitive EMDFF_Q(Q, D, CLK); // clock CLK is the last port
output Q;
input  D, CLK;
reg    Q;
table
	(??) ? : ? : -;
	0 (01) : ? : 0;
	1 (01) : ? : 1;
	? (10) : ? : -;
endtable
endprimitive
`celldefine
module EMDFF(M, Q, CLK, D);
output M, Q;
input  CLK, D;
specify
	(CLK => M) = (0.1, 0.1);
	(CLK => Q) = (0.1, 0.1);
	(D => M) = (0.1, 0.1);
	(D => Q) = (0.1, 0.1);
endspecify
EMDFF_M u_EMDFF_M (M, CLK, D);
EMDFF_Q u_EMDFF_Q (Q, D, CLK);
endmodule
`endcelldefine
primitive TAPDFF_Q(Q, D, CLK); // clock CLK is the last port
output Q;
input  D, CLK;
reg    Q;
table
	(??) ? : ? : -;
	0 (01) : ? : 0;
	1 (01) : ? : 1;
	? (10) : ? : -;
endtable
endprimitive
primitive TAPDFF_T(T, CLK, D);
output T;
input  CLK, D;
reg    T;
table
	0 0 : ? : 0;
	0 1 : ? : 1;
	1 ? : ? : -;
endtable
endprimitive
`celldefine
module TAPDFF(Q, T, CLK, D);
output Q, T;
input  CLK, D;
specify
	(CLK => Q) = (0.1, 0.1);
	(CLK => T) = (0.1, 0.1);
	(D => Q) = (0.1, 0.1);
	(D => T) = (0.1, 0.1);
endspecify
TAPDFF_Q u_TAPDFF_Q (Q, D, CLK);
TAPDFF_T u_TAPDFF_T (T, CLK, D);
endmodule
`endcelldefine
primitive IDFF_Q(Q, D, CLK); // clock CLK is the last port
output Q;
input  D, CLK;
reg    Q;
table
	(??) ? : ? : -;
	0 (01) : ? : 1;
	1 (01) : ? : 0;
	? (10) : ? : -;
endtable
endprimitive
`celldefine
module IDFF(Q, CLK, D);
output Q;
input  CLK, D;
specify
	(CLK => Q) = (0.1, 0.1);
	(D => Q) = (0.1, 0.1);
endspecify
IDFF_Q u_IDFF_Q (Q, D, CLK);
endmodule
`endcelldefine
primitive XN_Q(Q, D, CLK); // clock CLK is the last port
output Q;
input  D, CLK;
reg    Q;
table
	(??) ? : ? : -;
	0 (01) : ? : 0;
	1 (01) : ? : 1;
	? (10) : ? : -;
endtable
endprimitive
primitive XN_Qn(Qn, D, CLK); // clock CLK is the last port
output Qn;
input  D, CLK;
reg    Qn;
table
	(??) ? : ? : -;
	0 (01) : ? : 1;
	1 (01) : ? : 0;
	? (10) : ? : -;
endtable
endprimitive
`celldefine
module XN(Q, Qn, CLK, D);
output Q, Qn;
input  CLK, D;
specify
	(CLK => Q) = (0.1, 0.1);
	(CLK => Qn) = (0.1, 0.1);
	(D => Q) = (0.1, 0.1);
	(D => Qn) = (0.1, 0.1);
endspecify
XN_Q u_XN_Q (Q, D, CLK);
XN_Qn u_XN_Qn (Qn, D, CLK);
endmodule
`endcelldefine
primitive TFF_Q(Q, R, CLK); // clock CLK is the last port
output Q;
input  R, CLK;
reg    Q;
table
	(??) ? : ? : -;
	0 (01) : 0 : 1;
	1 (01) : ? : 0;
	1 ? : ? : 0;
	? (01) : 1 : 0;
	? (10) : ? : -;
endtable
endprimitive
primitive TFF_M(M, R, Q, CLK); // clock CLK is the last port
output M;
input  R, Q, CLK;
reg    M;
table
	(??) ? ? : ? : -;
	0 0 (10) : ? : 1;
	1 ? (10) : ? : 0;
	1 ? ? : ? : 0;
	? (??) ? : ? : -;
	? 1 (10) : ? : 0;
	? ? (01) : ? : -;
endtable
endprimitive
`celldefine
module TFF(Q, CLK, R);
output Q;
input  CLK, R;
wire   M;
specify
	(CLK => Q) = (0.1, 0.1);
	(R => Q) = (0.1, 0.1);
endspecify
TFF_Q u_TFF_Q (Q, R, CLK);
TFF_M u_TFF_M (M, R, Q, CLK);
endmodule
`endcelldefine
primitive DET_Q(Q, D, CLK); // clock CLK is the last port
output Q;
input  D, CLK;
reg    Q;
table
	(??) ? : ? : -;
	0 (01) : ? : 0;
	0 (10) : ? : 0;
	1 (01) : ? : 1;
	1 (10) : ? : 1;
endtable
endprimitive
`celldefine
module DET(Q, CLK, D);
output Q;
input  CLK, D;
specify
	(CLK => Q) = (0.1, 0.1);
	(D => Q) = (0.1, 0.1);
endspecify
DET_Q u_DET_Q (Q, D, CLK);
endmodule
`endcelldefine
primitive MOR_Q(Q, D, R, CLK); // clock CLK is the last port
output Q;
input  D, R, CLK;
reg    Q;
table
	(??) ? ? : ? : -;
	0 ? (01) : ? : 0;
	1 0 (01) : ? : 1;
	? (??) ? : ? : -;
	? 1 (01) : ? : 0;
	? 1 1 : ? : 0;
	? ? (10) : ? : -;
endtable
endprimitive
`celldefine
module MOR(Q, CLK, D, R);
output Q;
input  CLK, D, R;
specify
	(CLK => Q) = (0.1, 0.1);
	(D => Q) = (0.1, 0.1);
	(R => Q) = (0.1, 0.1);
endspecify
MOR_Q u_MOR_Q (Q, D, R, CLK);
endmodule
`endcelldefine
primitive MORA_Q(Q, D, R, CLK); // clock CLK is the last port
output Q;
input  D, R, CLK;
reg    Q;
table
	(??) ? ? : ? : -;
	0 ? (01) : ? : 0;
	1 0 (01) : ? : 1;
	? (??) ? : ? : -;
	? 1 (01) : ? : 0;
	? 1 1 : ? : 0;
	? ? (10) : ? : -;
endtable
endprimitive
`celldefine
module MORA(Q, CLK, D, R);
output Q;
input  CLK, D, R;
specify
	(CLK => Q) = (0.1, 0.1);
	(D => Q) = (0.1, 0.1);
	(R => Q) = (0.1, 0.1);
endspecify
MORA_Q u_MORA_Q (Q, D, R, CLK);
endmodule
`endcelldefine
primitive BR_Q(Q, D, R, CLK); // clock CLK is the last port
output Q;
input  D, R, CLK;
reg    Q;
table
	(??) ? ? : ? : -;
	0 ? (01) : ? : 0;
	1 0 (01) : ? : 1;
	? (??) ? : ? : -;
	? 1 (01) : ? : 0;
	? 1 ? : ? : 0;
	? ? (10) : ? : -;
endtable
endprimitive
`celldefine
module BR(Q, CLK, D, R);
output Q;
input  CLK, D, R;
specify
	(CLK => Q) = (0.1, 0.1);
	(D => Q) = (0.1, 0.1);
	(R => Q) = (0.1, 0.1);
endspecify
BR_Q u_BR_Q (Q, D, R, CLK);
endmodule
`endcelldefine
primitive SYNCR_Q(Q, D, R, CLK); // clock CLK is the last port
output Q;
input  D, R, CLK;
reg    Q;
table
	(??) ? ? : ? : -;
	0 ? (01) : ? : 0;
	1 0 (01) : ? : 1;
	? (??) ? : ? : -;
	? 1 (01) : ? : 0;
	? 1 ? : ? : 0;
	? ? (10) : ? : -;
endtable
endprimitive
`celldefine
module SYNCR(Q, CLK, D, R);
output Q;
input  CLK, D, R;
specify
	(CLK => Q) = (0.1, 0.1);
	(D => Q) = (0.1, 0.1);
	(R => Q) = (0.1, 0.1);
endspecify
SYNCR_Q u_SYNCR_Q (Q, D, R, CLK);
endmodule
`endcelldefine
primitive SYNCRG_Q(Q, D, R, G, CLK); // clock CLK is the last port
output Q;
input  D, R, G, CLK;
reg    Q;
table
	(??) ? ? ? : ? : -;
	0 ? ? (01) : ? : 0;
	1 0 0 (01) : ? : 1;
	? (??) ? ? : ? : -;
	? 1 ? (01) : ? : 0;
	? 1 ? ? : ? : 0;
	? ? (??) ? : ? : -;
	? ? 1 (01) : ? : 0;
	? ? 1 ? : ? : 0;
	? ? ? (10) : ? : -;
endtable
endprimitive
`celldefine
module SYNCRG(Q, CLK, D, R, G);
output Q;
input  CLK, D, R, G;
specify
	(CLK => Q) = (0.1, 0.1);
	(D => Q) = (0.1, 0.1);
	(R => Q) = (0.1, 0.1);
	(G => Q) = (0.1, 0.1);
endspecify
SYNCRG_Q u_SYNCRG_Q (Q, D, R, G, CLK);
endmodule
`endcelldefine
primitive GATEDR_Q(Q, D, R, G, CLK); // clock CLK is the last port
output Q;
input  D, R, G, CLK;
reg    Q;
table
	(??) ? ? ? : ? : -;
	0 ? ? (01) : ? : 0;
	1 0 ? (01) : ? : 1;
	1 ? 0 (01) : ? : 1;
	? (??) ? ? : ? : -;
	? 1 1 (01) : ? : 0;
	? 1 1 ? : ? : 0;
	? ? (??) ? : ? : -;
	? ? ? (10) : ? : -;
endtable
endprimitive
`celldefine
module GATEDR(Q, CLK, D, R, G);
output Q;
input  CLK, D, R, G;
specify
	(CLK => Q) = (0.1, 0.1);
	(D => Q) = (0.1, 0.1);
	(R => Q) = (0.1, 0.1);
	(G => Q) = (0.1, 0.1);
endspecify
GATEDR_Q u_GATEDR_Q (Q, D, R, G, CLK);
endmodule
`endcelldefine
primitive AGATEDR_Q(Q, D, R, G, CLK); // clock CLK is the last port
output Q;
input  D, R, G, CLK;
reg    Q;
table
	(??) ? ? ? : ? : -;
	0 ? ? (01) : ? : 0;
	1 0 ? (01) : ? : 1;
	1 ? 0 (01) : ? : 1;
	? (??) ? ? : ? : -;
	? 1 1 (01) : ? : 0;
	? 1 1 ? : ? : 0;
	? ? (??) ? : ? : -;
	? ? ? (10) : ? : -;
endtable
endprimitive
`celldefine
module AGATEDR(Q, CLK, D, R, G);
output Q;
input  CLK, D, R, G;
specify
	(CLK => Q) = (0.1, 0.1);
	(D => Q) = (0.1, 0.1);
	(R => Q) = (0.1, 0.1);
	(G => Q) = (0.1, 0.1);
endspecify
AGATEDR_Q u_AGATEDR_Q (Q, D, R, G, CLK);
endmodule
`endcelldefine
primitive RDFF_Q(Q, D, R, CLK); // clock CLK is the last port
output Q;
input  D, R, CLK;
reg    Q;
table
	(??) ? ? : ? : -;
	0 ? (01) : ? : 0;
	1 0 (01) : ? : 1;
	? (??) ? : ? : -;
	? 1 (01) : ? : 0;
	? 1 ? : ? : 0;
	? ? (10) : ? : -;
endtable
endprimitive
`celldefine
module RDFF(Q, CLK, D, R);
output Q;
input  CLK, D, R;
specify
	(CLK => Q) = (0.1, 0.1);
	(D => Q) = (0.1, 0.1);
	(R => Q) = (0.1, 0.1);
endspecify
RDFF_Q u_RDFF_Q (Q, D, R, CLK);
endmodule
`endcelldefine
primitive COEX_Q(Q, D, B, R, CLK); // clock CLK is the last port
output Q;
input  D, B, R, CLK;
reg    Q;
table
	(??) ? ? ? : ? : -;
	0 0 ? (01) : ? : 0;
	1 ? 0 (01) : ? : 1;
	? (??) ? ? : ? : -;
	? 1 0 (01) : ? : 1;
	? 1 0 ? : ? : 1;
	? ? (??) ? : ? : -;
	? ? 1 (01) : ? : 0;
	? ? 1 ? : ? : 0;
	? ? ? (10) : ? : -;
endtable
endprimitive
`celldefine
module COEX(Q, CLK, D, B, R);
output Q;
input  CLK, D, B, R;
specify
	(CLK => Q) = (0.1, 0.1);
	(D => Q) = (0.1, 0.1);
	(B => Q) = (0.1, 0.1);
	(R => Q) = (0.1, 0.1);
endspecify
COEX_Q u_COEX_Q (Q, D, B, R, CLK);
endmodule
`endcelldefine
primitive CAFF_Q(Q, D, PRE, CLR, CLK); // clock CLK is the last port
output Q;
input  D, PRE, CLR, CLK;
reg    Q;
table
	(??) ? ? ? : ? : -;
	0 0 ? (01) : ? : 0;
	1 ? 0 (01) : ? : 1;
	? (??) ? ? : ? : -;
	? 1 0 (01) : ? : 1;
	? 1 0 ? : ? : 1;
	? ? (??) ? : ? : -;
	? ? 1 (01) : ? : 0;
	? ? 1 ? : ? : 0;
	? ? ? (10) : ? : -;
endtable
endprimitive
`celldefine
module CAFF(Q, CLK, D, PRE, CLR);
output Q;
input  CLK, D, PRE, CLR;
specify
	(CLK => Q) = (0.1, 0.1);
	(D => Q) = (0.1, 0.1);
	(PRE => Q) = (0.1, 0.1);
	(CLR => Q) = (0.1, 0.1);
endspecify
CAFF_Q u_CAFF_Q (Q, D, PRE, CLR, CLK);
endmodule
`endcelldefine
primitive DLAT_Q(Q, CLK, D);
output Q;
input  CLK, D;
reg    Q;
table
	0 ? : ? : -;
	1 0 : ? : 0;
	1 1 : ? : 1;
endtable
endprimitive
`celldefine
module DLAT(Q, CLK, D);
output Q;
input  CLK, D;
specify
	(CLK => Q) = (0.1, 0.1);
	(D => Q) = (0.1, 0.1);
endspecify
DLAT_Q u_DLAT_Q (Q, CLK, D);
endmodule
`endcelldefine
primitive DLAT_EN_Q(Q, EN, D);
output Q;
input  EN, D;
reg    Q;
table
	0 ? : ? : -;
	1 0 : ? : 0;
	1 1 : ? : 1;
endtable
endprimitive
`celldefine
module DLAT_EN(Q, EN, D);
output Q;
input  EN, D;
specify
	(EN => Q) = (0.1, 0.1);
	(D => Q) = (0.1, 0.1);
endspecify
DLAT_EN_Q u_DLAT_EN_Q (Q, EN, D);
endmodule
`endcelldefine
primitive DLAT_E_Q(Q, E, D);
output Q;
input  E, D;
reg    Q;
table
	0 ? : ? : -;
	1 0 : ? : 0;
	1 1 : ? : 1;
endtable
endprimitive
`celldefine
module DLAT_E(Q, E, D);
output Q;
input  E, D;
specify
	(E => Q) = (0.1, 0.1);
	(D => Q) = (0.1, 0.1);
endspecify
DLAT_E_Q u_DLAT_E_Q (Q, E, D);
endmodule
`endcelldefine
primitive GLAT_Q(Q, CLK, D);
output Q;
input  CLK, D;
reg    Q;
table
	0 ? : ? : -;
	1 1 : ? : 1;
	? 0 : ? : -;
endtable
endprimitive
`celldefine
module GLAT(Q, CLK, D);
output Q;
input  CLK, D;
specify
	(CLK => Q) = (0.1, 0.1);
	(D => Q) = (0.1, 0.1);
endspecify
GLAT_Q u_GLAT_Q (Q, CLK, D);
endmodule
`endcelldefine
primitive MUXLAT_Q(Q, CLKA, D, CLKB);
output Q;
input  CLKA, D, CLKB;
reg    Q;
table
	0 ? 0 : ? : -;
	1 0 ? : ? : 0;
	1 1 ? : ? : 1;
	? 0 1 : ? : 0;
	? 1 1 : ? : 1;
endtable
endprimitive
`celldefine
module MUXLAT(Q, CLKA, CLKB, D);
output Q;
input  CLKA, CLKB, D;
specify
	(CLKA => Q) = (0.1, 0.1);
	(CLKB => Q) = (0.1, 0.1);
	(D => Q) = (0.1, 0.1);
endspecify
MUXLAT_Q u_MUXLAT_Q (Q, CLKA, D, CLKB);
endmodule
`endcelldefine
primitive MCDFF_Q(Q, CLKB, M);
output Q;
input  CLKB, M;
reg    Q;
table
	0 ? : ? : -;
	1 0 : ? : 0;
	1 1 : ? : 1;
endtable
endprimitive
primitive MCDFF_M(M, CLKA, D);
output M;
input  CLKA, D;
reg    M;
table
	0 0 : ? : 0;
	0 1 : ? : 1;
	1 ? : ? : -;
endtable
endprimitive
`celldefine
module MCDFF(Q, CLKA, CLKB, D);
output Q;
input  CLKA, CLKB, D;
wire   M;
specify
	(CLKA => Q) = (0.1, 0.1);
	(CLKB => Q) = (0.1, 0.1);
	(D => Q) = (0.1, 0.1);
endspecify
MCDFF_Q u_MCDFF_Q (Q, CLKB, M);
MCDFF_M u_MCDFF_M (M, CLKA, D);
endmodule
`endcelldefine
primitive TCASC_Q(Q, CLK, M);
output Q;
input  CLK, M;
reg    Q;
table
	0 0 : ? : 0;
	0 1 : ? : 1;
	1 ? : ? : -;
endtable
endprimitive
primitive TCASC_M(M, CLK, D);
output M;
input  CLK, D;
reg    M;
table
	0 0 : ? : 0;
	0 1 : ? : 1;
	1 ? : ? : -;
endtable
endprimitive
`celldefine
module TCASC(Q, CLK, D);
output Q;
input  CLK, D;
wire   M;
specify
	(CLK => Q) = (0.1, 0.1);
	(D => Q) = (0.1, 0.1);
endspecify
TCASC_Q u_TCASC_Q (Q, CLK, M);
TCASC_M u_TCASC_M (M, CLK, D);
endmodule
`endcelldefine
primitive XLAT_T(T, M, M2);
output T;
input  M, M2;
reg    T;
table
	0 0 : ? : 0;
	0 1 : ? : 1;
	1 0 : ? : 1;
	1 1 : ? : 0;
endtable
endprimitive
primitive XLAT_M(M, CLK, D);
output M;
input  CLK, D;
reg    M;
table
	0 0 : ? : 0;
	0 1 : ? : 1;
	1 ? : ? : -;
endtable
endprimitive
primitive XLAT_M2(M2, CLK, D);
output M2;
input  CLK, D;
reg    M2;
table
	0 ? : ? : -;
	1 0 : ? : 0;
	1 1 : ? : 1;
endtable
endprimitive
`celldefine
module XLAT(T, CLK, D);
output T;
input  CLK, D;
wire   M, M2;
specify
	(CLK => T) = (0.1, 0.1);
	(D => T) = (0.1, 0.1);
endspecify
XLAT_T u_XLAT_T (T, M, M2);
XLAT_M u_XLAT_M (M, CLK, D);
XLAT_M2 u_XLAT_M2 (M2, CLK, D);
endmodule
`endcelldefine
primitive HPIPE_Q(Q, D, M2, CLKA, CLKB); // clocks CLKA, CLKB are the last ports
output Q;
input  D, M2, CLKA, CLKB;
reg    Q;
table
	(??) ? ? ? : ? : -;
	0 ? (01) 0 : ? : 0;
	1 ? (01) 0 : ? : 1;
	? (??) ? ? : ? : -;
	? 0 ? (10) : ? : 0;
	? 1 ? (10) : ? : 1;
	? ? (01) 1 : 0 : 0;
	? ? (01) 1 : 1 : 1;
	? ? (10) ? : ? : -;
	? ? ? (01) : ? : -;
endtable
endprimitive
primitive HPIPE_M2(M2, D, CLKA); // clock CLKA is the last port
output M2;
input  D, CLKA;
reg    M2;
table
	(??) ? : ? : -;
	0 (01) : ? : 0;
	1 (01) : ? : 1;
	? (10) : ? : -;
endtable
endprimitive
`celldefine
module HPIPE(Q, CLKA, CLKB, D);
output Q;
input  CLKA, CLKB, D;
wire   M2;
specify
	(CLKA => Q) = (0.1, 0.1);
	(CLKB => Q) = (0.1, 0.1);
	(D => Q) = (0.1, 0.1);
endspecify
HPIPE_Q u_HPIPE_Q (Q, D, M2, CLKA, CLKB);
HPIPE_M2 u_HPIPE_M2 (M2, D, CLKA);
endmodule
`endcelldefine
primitive DCMUX_Q(Q, CLKA, MA, CLKB, MB);
output Q;
input  CLKA, MA, CLKB, MB;
reg    Q;
table
	0 ? 0 ? : ? : -;
	0 ? 1 0 : ? : 0;
	1 0 0 ? : ? : 0;
	1 1 ? ? : ? : 1;
	? 0 1 0 : ? : 0;
	? ? 1 1 : ? : 1;
endtable
endprimitive
primitive DCMUX_MA(MA, CLKA, DA);
output MA;
input  CLKA, DA;
reg    MA;
table
	0 0 : ? : 0;
	0 1 : ? : 1;
	1 ? : ? : -;
endtable
endprimitive
primitive DCMUX_MB(MB, CLKB, DB);
output MB;
input  CLKB, DB;
reg    MB;
table
	0 0 : ? : 0;
	0 1 : ? : 1;
	1 ? : ? : -;
endtable
endprimitive
`celldefine
module DCMUX(Q, CLKA, CLKB, DA, DB);
output Q;
input  CLKA, CLKB, DA, DB;
wire   MA, MB;
specify
	(CLKA => Q) = (0.1, 0.1);
	(CLKB => Q) = (0.1, 0.1);
	(DA => Q) = (0.1, 0.1);
	(DB => Q) = (0.1, 0.1);
endspecify
DCMUX_Q u_DCMUX_Q (Q, CLKA, MA, CLKB, MB);
DCMUX_MA u_DCMUX_MA (MA, CLKA, DA);
DCMUX_MB u_DCMUX_MB (MB, CLKB, DB);
endmodule
`endcelldefine
primitive ICG_GCLK(GCLK, CLK, EL);
output GCLK;
input  CLK, EL;
reg    GCLK;
table
	0 ? : ? : 0;
	1 1 : ? : 1;
	? 0 : ? : 0;
endtable
endprimitive
primitive ICG_EL(EL, CLK, EN);
output EL;
input  CLK, EN;
reg    EL;
table
	0 0 : ? : 0;
	0 1 : ? : 1;
	1 ? : ? : -;
endtable
endprimitive
`celldefine
module ICG(GCLK, CLK, EN);
output GCLK;
input  CLK, EN;
wire   EL;
specify
	(CLK => GCLK) = (0.1, 0.1);
	(EN => GCLK) = (0.1, 0.1);
endspecify
ICG_GCLK u_ICG_GCLK (GCLK, CLK, EL);
ICG_EL u_ICG_EL (EL, CLK, EN);
endmodule
`endcelldefine
primitive ICM_GCLK(GCLK, enA, CLKA, enB, CLKB);
output GCLK;
input  enA, CLKA, enB, CLKB;
reg    GCLK;
table
	0 ? 0 ? : ? : 0;
	0 ? ? 0 : ? : 0;
	1 1 ? ? : ? : 1;
	? 0 0 ? : ? : 0;
	? 0 ? 0 : ? : 0;
	? ? 1 1 : ? : 1;
endtable
endprimitive
primitive ICM_enA(enA, sela2, RA, CLKA); // clock CLKA is the last port
output enA;
input  sela2, RA, CLKA;
reg    enA;
table
	(??) ? ? : ? : -;
	0 ? (10) : ? : 0;
	1 ? (10) : ? : 1;
	? (??) ? : ? : -;
	? 1 ? : ? : 0;
	? ? (01) : ? : -;
endtable
endprimitive
primitive ICM_enB(enB, selb2, RB, CLKB); // clock CLKB is the last port
output enB;
input  selb2, RB, CLKB;
reg    enB;
table
	(??) ? ? : ? : -;
	0 ? (10) : ? : 0;
	1 ? (10) : ? : 1;
	? (??) ? : ? : -;
	? 1 ? : ? : 0;
	? ? (01) : ? : -;
endtable
endprimitive
primitive ICM_sela2(sela2, RA, S, enB, CLKA); // clock CLKA is the last port
output sela2;
input  RA, S, enB, CLKA;
reg    sela2;
table
	(??) ? ? ? : ? : -;
	0 0 0 (01) : ? : 1;
	1 ? ? (01) : ? : 0;
	1 ? ? ? : ? : 0;
	? (??) ? ? : ? : -;
	? 1 ? (01) : ? : 0;
	? ? (??) ? : ? : -;
	? ? 1 (01) : ? : 0;
	? ? ? (10) : ? : -;
endtable
endprimitive
primitive ICM_selb2(selb2, RB, S, enA, CLKB); // clock CLKB is the last port
output selb2;
input  RB, S, enA, CLKB;
reg    selb2;
table
	(??) ? ? ? : ? : -;
	0 1 0 (01) : ? : 1;
	1 ? ? (01) : ? : 0;
	1 ? ? ? : ? : 0;
	? (??) ? ? : ? : -;
	? 0 ? (01) : ? : 0;
	? ? (??) ? : ? : -;
	? ? 1 (01) : ? : 0;
	? ? ? (10) : ? : -;
endtable
endprimitive
`celldefine
module ICM(GCLK, CLKA, CLKB, RA, RB, S);
output GCLK;
input  CLKA, CLKB, RA, RB, S;
wire   enA, enB, sela2, selb2;
specify
	(CLKA => GCLK) = (0.1, 0.1);
	(CLKB => GCLK) = (0.1, 0.1);
	(RA => GCLK) = (0.1, 0.1);
	(RB => GCLK) = (0.1, 0.1);
	(S => GCLK) = (0.1, 0.1);
endspecify
ICM_GCLK u_ICM_GCLK (GCLK, enA, CLKA, enB, CLKB);
ICM_enA u_ICM_enA (enA, sela2, RA, CLKA);
ICM_enB u_ICM_enB (enB, selb2, RB, CLKB);
ICM_sela2 u_ICM_sela2 (sela2, RA, S, enB, CLKA);
ICM_selb2 u_ICM_selb2 (selb2, RB, S, enA, CLKB);
endmodule
`endcelldefine
primitive GL_Y(Y, C, L);
output Y;
input  C, L;
reg    Y;
table
	0 ? : ? : 0;
	1 1 : ? : 1;
	? 0 : ? : 0;
endtable
endprimitive
primitive GL_L(L, C, D);
output L;
input  C, D;
reg    L;
table
	0 0 : ? : 0;
	0 1 : ? : 1;
	1 ? : ? : -;
endtable
endprimitive
`celldefine
module GL(Y, C, D);
output Y;
input  C, D;
wire   L;
specify
	(C => Y) = (0.1, 0.1);
	(D => Y) = (0.1, 0.1);
endspecify
GL_Y u_GL_Y (Y, C, L);
GL_L u_GL_L (L, C, D);
endmodule
`endcelldefine
primitive MIX_Y(Y, C, L);
output Y;
input  C, L;
reg    Y;
table
	0 ? : ? : 0;
	1 1 : ? : 1;
	? 0 : ? : 0;
endtable
endprimitive
primitive MIX_Z(Z, A, B);
output Z;
input  A, B;
reg    Z;
table
	0 ? : ? : 0;
	1 1 : ? : 1;
	? 0 : ? : 0;
endtable
endprimitive
primitive MIX_L(L, C, D);
output L;
input  C, D;
reg    L;
table
	0 0 : ? : 0;
	0 1 : ? : 1;
	1 ? : ? : -;
endtable
endprimitive
`celldefine
module MIX(Y, Z, A, B, C, D);
output Y, Z;
input  A, B, C, D;
wire   L;
specify
	(A => Y) = (0.1, 0.1);
	(A => Z) = (0.1, 0.1);
	(B => Y) = (0.1, 0.1);
	(B => Z) = (0.1, 0.1);
	(C => Y) = (0.1, 0.1);
	(C => Z) = (0.1, 0.1);
	(D => Y) = (0.1, 0.1);
	(D => Z) = (0.1, 0.1);
endspecify
MIX_Y u_MIX_Y (Y, C, L);
MIX_Z u_MIX_Z (Z, A, B);
MIX_L u_MIX_L (L, C, D);
endmodule
`endcelldefine
primitive TRW_Z2(Z2, E, C, L);
output Z2;
input  E, C, L;
reg    Z2;
table
	0 0 ? : ? : 0;
	0 ? 0 : ? : 0;
	1 ? ? : ? : 1;
	? 1 1 : ? : 1;
endtable
endprimitive
primitive TRW_L(L, C, D);
output L;
input  C, D;
reg    L;
table
	0 0 : ? : 0;
	0 1 : ? : 1;
	1 ? : ? : -;
endtable
endprimitive
`celldefine
module TRW(Z2, C, D, E);
output Z2;
input  C, D, E;
wire   L;
specify
	(C => Z2) = (0.1, 0.1);
	(D => Z2) = (0.1, 0.1);
	(E => Z2) = (0.1, 0.1);
endspecify
TRW_Z2 u_TRW_Z2 (Z2, E, C, L);
TRW_L u_TRW_L (L, C, D);
endmodule
`endcelldefine
primitive COLL_Q(Q, A, Q_st);
output Q;
input  A, Q_st;
reg    Q;
table
	0 0 : ? : 0;
	0 1 : ? : -;
	1 0 : ? : -;
	1 1 : ? : 1;
endtable
endprimitive
`celldefine
module COLL(Q, A, Q_st);
output Q;
input  A, Q_st;
specify
	(A => Q) = (0.1, 0.1);
	(Q_st => Q) = (0.1, 0.1);
endspecify
COLL_Q u_COLL_Q (Q, A, Q_st);
endmodule
`endcelldefine
primitive C2P_Q(Q, A, B);
output Q;
input  A, B;
reg    Q;
table
	0 0 : ? : 0;
	0 1 : ? : -;
	1 0 : ? : -;
	1 1 : ? : 1;
endtable
endprimitive
primitive C2P_Qc(Qc, Q);
output Qc;
input  Q;
reg    Qc;
table
	0 : ? : 0;
	1 : ? : 1;
endtable
endprimitive
primitive C2P_Qn(Qn, Q);
output Qn;
input  Q;
reg    Qn;
table
	0 : ? : 1;
	1 : ? : 0;
endtable
endprimitive
`celldefine
module C2P(Q, Qc, Qn, A, B);
output Q, Qc, Qn;
input  A, B;
specify
	(A => Q) = (0.1, 0.1);
	(A => Qc) = (0.1, 0.1);
	(A => Qn) = (0.1, 0.1);
	(B => Q) = (0.1, 0.1);
	(B => Qc) = (0.1, 0.1);
	(B => Qn) = (0.1, 0.1);
endspecify
C2P_Q u_C2P_Q (Q, A, B);
C2P_Qc u_C2P_Qc (Qc, Q);
C2P_Qn u_C2P_Qn (Qn, Q);
endmodule
`endcelldefine
primitive RC2_Q(Q, A, B, R);
output Q;
input  A, B, R;
reg    Q;
table
	0 0 ? : ? : 0;
	0 1 0 : ? : -;
	1 0 0 : ? : -;
	1 1 0 : ? : 1;
	? ? 1 : ? : 0;
endtable
endprimitive
`celldefine
module RC2(Q, A, B, R);
output Q;
input  A, B, R;
specify
	(A => Q) = (0.1, 0.1);
	(B => Q) = (0.1, 0.1);
	(R => Q) = (0.1, 0.1);
endspecify
RC2_Q u_RC2_Q (Q, A, B, R);
endmodule
`endcelldefine
