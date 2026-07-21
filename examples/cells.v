primitive AND2_Y(Y, A, B);
output Y;
input  A, B;
reg    Y;
table
	0 ? : ? : 0;
	1 1 : ? : 1;
	? 0 : ? : 0;
endtable
endprimitive
`celldefine
module AND2(Y, A, B);
output Y;
input  A, B;
specify
	(A => Y) = (0.1, 0.1);
	(B => Y) = (0.1, 0.1);
endspecify
AND2_Y u_AND2_Y (Y, A, B);
endmodule
`endcelldefine
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
primitive SR_Q(Q, R, Qn);
output Q;
input  R, Qn;
reg    Q;
table
	0 0 : ? : 1;
	1 ? : ? : 0;
	? 1 : ? : 0;
endtable
endprimitive
primitive SR_Qn(Qn, S, Q);
output Qn;
input  S, Q;
reg    Qn;
table
	0 0 : ? : 1;
	1 ? : ? : 0;
	? 1 : ? : 0;
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
SR_Q u_SR_Q (Q, R, Qn);
SR_Qn u_SR_Qn (Qn, S, Q);
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
primitive DLH_Q(Q, G, D);
output Q;
input  G, D;
reg    Q;
table
	0 ? : ? : -;
	1 0 : ? : 0;
	1 1 : ? : 1;
endtable
endprimitive
`celldefine
module DLH(Q, G, D);
output Q;
input  G, D;
specify
	(G => Q) = (0.1, 0.1);
	(D => Q) = (0.1, 0.1);
endspecify
DLH_Q u_DLH_Q (Q, G, D);
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
primitive C2GATE_Q(Q, A, B);
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
module C2GATE(Q, A, B);
output Q;
input  A, B;
specify
	(A => Q) = (0.1, 0.1);
	(B => Q) = (0.1, 0.1);
endspecify
C2GATE_Q u_C2GATE_Q (Q, A, B);
endmodule
`endcelldefine
