primitive C2_Q(Q, A, B);
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
`celldefine
module C2(Q, A, B);
output Q;
input  A, B;
specify
	(A => Q) = (0.1, 0.1);
	(B => Q) = (0.1, 0.1);
endspecify
C2_Q u_C2_Q (Q, A, B);
endmodule
`endcelldefine
primitive RCELEM2_Q(Q, A, B, R);
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
module RCELEM2(Q, A, B, R);
output Q;
input  A, B, R;
specify
	(A => Q) = (0.1, 0.1);
	(B => Q) = (0.1, 0.1);
	(R => Q) = (0.1, 0.1);
endspecify
RCELEM2_Q u_RCELEM2_Q (Q, A, B, R);
endmodule
`endcelldefine
primitive RACELEM21_Q(Q, M1, M2, P1, P2, C, R);
output Q;
input  M1, M2, P1, P2, C, R;
reg    Q;
table
	0 0 0 ? ? ? : ? : 0;
	0 0 ? 0 ? ? : ? : 0;
	0 0 ? ? 0 ? : ? : 0;
	1 ? 0 ? ? 0 : ? : -;
	1 ? ? 0 ? 0 : ? : -;
	1 ? ? ? 0 0 : ? : -;
	? 1 0 ? ? 0 : ? : -;
	? 1 ? 0 ? 0 : ? : -;
	? 1 ? ? 0 0 : ? : -;
	? ? 1 1 1 0 : ? : 1;
	? ? ? ? ? 1 : ? : 0;
endtable
endprimitive
`celldefine
module RACELEM21(Q, M1, M2, P1, P2, C, R);
output Q;
input  M1, M2, P1, P2, C, R;
specify
	(M1 => Q) = (0.1, 0.1);
	(M2 => Q) = (0.1, 0.1);
	(P1 => Q) = (0.1, 0.1);
	(P2 => Q) = (0.1, 0.1);
	(C => Q) = (0.1, 0.1);
	(R => Q) = (0.1, 0.1);
endspecify
RACELEM21_Q u_RACELEM21_Q (Q, M1, M2, P1, P2, C, R);
endmodule
`endcelldefine
primitive SR_Q(Q, S, R);
output Q;
input  S, R;
reg    Q;
table
	0 0 : ? : -;
	0 1 : ? : 0;
	1 ? : ? : 1;
endtable
endprimitive
primitive SR_Qn(Qn, S, R);
output Qn;
input  S, R;
reg    Qn;
table
	0 0 : ? : -;
	1 0 : ? : 0;
	? 1 : ? : 1;
endtable
endprimitive
`celldefine
module SR(Q, Qn, S, R);
output Q, Qn;
input  S, R;
specify
	(S => Q) = (0.1, 0.1);
	(S => Qn) = (0.1, 0.1);
	(R => Q) = (0.1, 0.1);
	(R => Qn) = (0.1, 0.1);
endspecify
SR_Q u_SR_Q (Q, S, R);
SR_Qn u_SR_Qn (Qn, S, R);
endmodule
`endcelldefine
primitive MUT_Qa(Qa, A, B, Qb);
output Qa;
input  A, B, Qb;
reg    Qa;
table
	0 ? ? : ? : 0;
	1 ? 0 : ? : 1;
	? ? 1 : ? : 0;
endtable
endprimitive
primitive MUT_Qb(Qb, A, B, Qa);
output Qb;
input  A, B, Qa;
reg    Qb;
table
	? 0 ? : ? : 0;
	? 1 0 : ? : 1;
	? ? 1 : ? : 0;
endtable
endprimitive
`celldefine
module MUT(Qa, Qb, A, B);
output Qa, Qb;
input  A, B;
specify
	(A => Qa) = (0.1, 0.1);
	(A => Qb) = (0.1, 0.1);
	(B => Qa) = (0.1, 0.1);
	(B => Qb) = (0.1, 0.1);
endspecify
MUT_Qa u_MUT_Qa (Qa, A, B, Qb);
MUT_Qb u_MUT_Qb (Qb, A, B, Qa);
endmodule
`endcelldefine
primitive DFF_Q(Q, CLK, D, M);
output Q;
input  CLK, D, M;
reg    Q;
table
	0 ? ? : ? : -;
	1 ? 0 : ? : 0;
	1 ? 1 : ? : 1;
endtable
endprimitive
primitive DFF_M(M, CLK, D);
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
module DFF(Q, CLK, D);
output Q;
input  CLK, D;
wire   M;
specify
	(CLK => Q) = (0.1, 0.1);
	(D => Q) = (0.1, 0.1);
endspecify
DFF_Q u_DFF_Q (Q, CLK, D, M);
DFF_M u_DFF_M (M, CLK, D);
endmodule
`endcelldefine
primitive ICM_GCLK(GCLK, CLKA, CLKB, RA, RB, S, enA, enB);
output GCLK;
input  CLKA, CLKB, RA, RB, S, enA, enB;
reg    GCLK;
table
	0 0 ? ? ? ? ? : ? : 0;
	0 ? ? ? ? ? 0 : ? : 0;
	1 ? ? ? ? 1 ? : ? : 1;
	? 0 ? ? ? 0 ? : ? : 0;
	? 1 ? ? ? ? 1 : ? : 1;
	? ? ? ? ? 0 0 : ? : 0;
endtable
endprimitive
primitive ICM_sela(sela, CLKA, CLKB, RA, RB, S, enB);
output sela;
input  CLKA, CLKB, RA, RB, S, enB;
reg    sela;
table
	? ? ? ? 0 0 : ? : 1;
	? ? ? ? 1 ? : ? : 0;
	? ? ? ? ? 1 : ? : 0;
endtable
endprimitive
primitive ICM_selb(selb, CLKA, CLKB, RA, RB, S, enA);
output selb;
input  CLKA, CLKB, RA, RB, S, enA;
reg    selb;
table
	? ? ? ? 0 ? : ? : 0;
	? ? ? ? 1 0 : ? : 1;
	? ? ? ? ? 1 : ? : 0;
endtable
endprimitive
primitive ICM_sela1(sela1, CLKA, CLKB, RA, RB, S, sela);
output sela1;
input  CLKA, CLKB, RA, RB, S, sela;
reg    sela1;
table
	0 ? 0 ? ? 1 : ? : 1;
	0 ? ? ? ? 0 : ? : 0;
	1 ? 0 ? ? ? : ? : -;
	? ? 1 ? ? ? : ? : 0;
endtable
endprimitive
primitive ICM_sela2(sela2, CLKA, CLKB, RA, RB, S, sela1);
output sela2;
input  CLKA, CLKB, RA, RB, S, sela1;
reg    sela2;
table
	0 ? 0 ? ? ? : ? : -;
	1 ? 0 ? ? 1 : ? : 1;
	1 ? ? ? ? 0 : ? : 0;
	? ? 1 ? ? ? : ? : 0;
endtable
endprimitive
primitive ICM_enA(enA, CLKA, CLKB, RA, RB, S, sela2);
output enA;
input  CLKA, CLKB, RA, RB, S, sela2;
reg    enA;
table
	0 ? 0 ? ? 1 : ? : 1;
	0 ? ? ? ? 0 : ? : 0;
	1 ? 0 ? ? ? : ? : -;
	? ? 1 ? ? ? : ? : 0;
endtable
endprimitive
primitive ICM_selb1(selb1, CLKA, CLKB, RA, RB, S, selb);
output selb1;
input  CLKA, CLKB, RA, RB, S, selb;
reg    selb1;
table
	? 0 ? 0 ? 1 : ? : 1;
	? 0 ? ? ? 0 : ? : 0;
	? 1 ? 0 ? ? : ? : -;
	? ? ? 1 ? ? : ? : 0;
endtable
endprimitive
primitive ICM_selb2(selb2, CLKA, CLKB, RA, RB, S, selb1);
output selb2;
input  CLKA, CLKB, RA, RB, S, selb1;
reg    selb2;
table
	? 0 ? 0 ? ? : ? : -;
	? 1 ? 0 ? 1 : ? : 1;
	? 1 ? ? ? 0 : ? : 0;
	? ? ? 1 ? ? : ? : 0;
endtable
endprimitive
primitive ICM_enB(enB, CLKA, CLKB, RA, RB, S, selb2);
output enB;
input  CLKA, CLKB, RA, RB, S, selb2;
reg    enB;
table
	? 0 ? 0 ? 1 : ? : 1;
	? 0 ? ? ? 0 : ? : 0;
	? 1 ? 0 ? ? : ? : -;
	? ? ? 1 ? ? : ? : 0;
endtable
endprimitive
`celldefine
module ICM(GCLK, CLKA, CLKB, RA, RB, S);
output GCLK;
input  CLKA, CLKB, RA, RB, S;
wire   sela, selb, sela1, sela2, enA, selb1, selb2, enB;
specify
	(CLKA => GCLK) = (0.1, 0.1);
	(CLKB => GCLK) = (0.1, 0.1);
	(RA => GCLK) = (0.1, 0.1);
	(RB => GCLK) = (0.1, 0.1);
	(S => GCLK) = (0.1, 0.1);
endspecify
ICM_GCLK u_ICM_GCLK (GCLK, CLKA, CLKB, RA, RB, S, enA, enB);
ICM_sela u_ICM_sela (sela, CLKA, CLKB, RA, RB, S, enB);
ICM_selb u_ICM_selb (selb, CLKA, CLKB, RA, RB, S, enA);
ICM_sela1 u_ICM_sela1 (sela1, CLKA, CLKB, RA, RB, S, sela);
ICM_sela2 u_ICM_sela2 (sela2, CLKA, CLKB, RA, RB, S, sela1);
ICM_enA u_ICM_enA (enA, CLKA, CLKB, RA, RB, S, sela2);
ICM_selb1 u_ICM_selb1 (selb1, CLKA, CLKB, RA, RB, S, selb);
ICM_selb2 u_ICM_selb2 (selb2, CLKA, CLKB, RA, RB, S, selb1);
ICM_enB u_ICM_enB (enB, CLKA, CLKB, RA, RB, S, selb2);
endmodule
`endcelldefine
