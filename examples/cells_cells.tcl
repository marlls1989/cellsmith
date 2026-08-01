define_cell \
	-input { A B } \
	-output { Y } \
	-pinlist { A B Y } \
	{ AND2 }

define_cell \
	-input { A } \
	-output { Y } \
	-pinlist { A Y } \
	-delay inv_delay \
	-power inv_power \
	-constrain inv_constrain \
	{ INVX1 INVX3 }

define_cell \
	-input { A } \
	-output { Y } \
	-pinlist { A Y } \
	-delay inv_delay_x2 \
	-power inv_power \
	-constrain inv_constrain \
	{ INVX2 }

define_cell \
	-input { A B } \
	-output { Q } \
	-pinlist { A B Q } \
	{ C2 }

define_cell \
	-input { A B R } \
	-output { Q } \
	-pinlist { A B R Q } \
	{ RCELEM2 }

define_cell \
	-input { M1 M2 P1 P2 C R } \
	-output { Q } \
	-pinlist { M1 M2 P1 P2 C R Q } \
	{ RACELEM21 }

define_cell \
	-input { S R } \
	-output { Q Qn } \
	-pinlist { S R Q Qn } \
	{ SR }

define_cell \
	-input { A B } \
	-output { Qa Qb } \
	-pinlist { A B Qa Qb } \
	{ MUT }

define_cell \
	-input { D } \
	-output { Q } \
	-clock { CLK } \
	-pinlist { CLK D Q } \
	{ DFF }

define_cell \
	-input { D } \
	-output { Q } \
	-clock { G } \
	-pinlist { G D Q } \
	{ DLH }

define_cell \
	-input { RA RB S } \
	-output { GCLK } \
	-clock { CLKA CLKB } \
	-pinlist { CLKA CLKB RA RB S GCLK } \
	{ ICM }

define_cell \
	-input { A B } \
	-output { Q } \
	-pinlist { A B Q } \
	{ C2GATE }

