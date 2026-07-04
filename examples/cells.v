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
primitive RACELEM21_Q(Q, P1, P2, C, R, M1, M2);
output Q;
input  P1, P2, C, R, M1, M2;
reg    Q;
table
	0 ? 1 0 ? ? : ? : -;
	1 1 1 0 ? ? : ? : 1;
	? 0 1 0 ? ? : ? : -;
	? ? 0 0 1 ? : ? : -;
	? ? 0 0 ? 1 : ? : -;
	? ? 0 ? 0 0 : ? : 0;
	? ? ? 1 ? ? : ? : 0;
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
RACELEM21_Q u_RACELEM21_Q (Q, P1, P2, C, R, M1, M2);
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
primitive SR_Qn(Qn, R, S);
output Qn;
input  R, S;
reg    Qn;
table
	0 0 : ? : -;
	0 1 : ? : 0;
	1 ? : ? : 1;
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
SR_Qn u_SR_Qn (Qn, R, S);
endmodule
`endcelldefine
primitive MUT_Qa(Qa, Qb, A);
output Qa;
input  Qb, A;
reg    Qa;
table
	0 1 : ? : 1;
	1 ? : ? : 0;
	? 0 : ? : 0;
endtable
endprimitive
primitive MUT_Qb(Qb, Qa, B);
output Qb;
input  Qa, B;
reg    Qb;
table
	0 1 : ? : 1;
	1 ? : ? : 0;
	? 0 : ? : 0;
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
MUT_Qa u_MUT_Qa (Qa, Qb, A);
MUT_Qb u_MUT_Qb (Qb, Qa, B);
endmodule
`endcelldefine
primitive DFF_Q(Q, CLK, M);
output Q;
input  CLK, M;
reg    Q;
table
	0 ? : ? : -;
	1 0 : ? : 0;
	1 1 : ? : 1;
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
DFF_Q u_DFF_Q (Q, CLK, M);
DFF_M u_DFF_M (M, CLK, D);
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
primitive ICM_enA(enA, RA, CLKA, sela2);
output enA;
input  RA, CLKA, sela2;
reg    enA;
table
	0 0 1 : ? : 1;
	0 1 ? : ? : -;
	1 ? ? : ? : 0;
	? 0 0 : ? : 0;
endtable
endprimitive
primitive ICM_enB(enB, RB, CLKB, selb2);
output enB;
input  RB, CLKB, selb2;
reg    enB;
table
	0 0 1 : ? : 1;
	0 1 ? : ? : -;
	1 ? ? : ? : 0;
	? 0 0 : ? : 0;
endtable
endprimitive
primitive ICM_sela(sela, enB, S);
output sela;
input  enB, S;
reg    sela;
table
	0 0 : ? : 1;
	1 ? : ? : 0;
	? 1 : ? : 0;
endtable
endprimitive
primitive ICM_sela1(sela1, RA, CLKA, sela);
output sela1;
input  RA, CLKA, sela;
reg    sela1;
table
	0 0 1 : ? : 1;
	0 1 ? : ? : -;
	1 ? ? : ? : 0;
	? 0 0 : ? : 0;
endtable
endprimitive
primitive ICM_sela2(sela2, RA, CLKA, sela1);
output sela2;
input  RA, CLKA, sela1;
reg    sela2;
table
	0 0 ? : ? : -;
	0 1 1 : ? : 1;
	1 ? ? : ? : 0;
	? 1 0 : ? : 0;
endtable
endprimitive
primitive ICM_selb(selb, enA, S);
output selb;
input  enA, S;
reg    selb;
table
	0 1 : ? : 1;
	1 ? : ? : 0;
	? 0 : ? : 0;
endtable
endprimitive
primitive ICM_selb1(selb1, RB, CLKB, selb);
output selb1;
input  RB, CLKB, selb;
reg    selb1;
table
	0 0 1 : ? : 1;
	0 1 ? : ? : -;
	1 ? ? : ? : 0;
	? 0 0 : ? : 0;
endtable
endprimitive
primitive ICM_selb2(selb2, RB, CLKB, selb1);
output selb2;
input  RB, CLKB, selb1;
reg    selb2;
table
	0 0 ? : ? : -;
	0 1 1 : ? : 1;
	1 ? ? : ? : 0;
	? 1 0 : ? : 0;
endtable
endprimitive
`celldefine
module ICM(GCLK, CLKA, CLKB, RA, RB, S);
output GCLK;
input  CLKA, CLKB, RA, RB, S;
wire   enA, enB, sela, sela1, sela2, selb, selb1, selb2;
specify
	(CLKA => GCLK) = (0.1, 0.1);
	(CLKB => GCLK) = (0.1, 0.1);
	(RA => GCLK) = (0.1, 0.1);
	(RB => GCLK) = (0.1, 0.1);
	(S => GCLK) = (0.1, 0.1);
endspecify
ICM_GCLK u_ICM_GCLK (GCLK, enA, CLKA, enB, CLKB);
ICM_enA u_ICM_enA (enA, RA, CLKA, sela2);
ICM_enB u_ICM_enB (enB, RB, CLKB, selb2);
ICM_sela u_ICM_sela (sela, enB, S);
ICM_sela1 u_ICM_sela1 (sela1, RA, CLKA, sela);
ICM_sela2 u_ICM_sela2 (sela2, RA, CLKA, sela1);
ICM_selb u_ICM_selb (selb, enA, S);
ICM_selb1 u_ICM_selb1 (selb1, RB, CLKB, selb);
ICM_selb2 u_ICM_selb2 (selb2, RB, CLKB, selb1);
endmodule
`endcelldefine
