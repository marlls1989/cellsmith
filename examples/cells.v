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
	0 1 1 : ? : 0;
	1 0 0 : ? : -;
	1 1 0 : ? : 1;
	1 ? 1 : ? : 0;
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
	0 ? 0 : ? : 0;
	1 ? 0 : ? : 1;
	? ? 1 : ? : 0;
endtable
endprimitive
primitive MUT_Qb(Qb, A, B, Qa);
output Qb;
input  A, B, Qa;
reg    Qb;
table
	? 0 0 : ? : 0;
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
