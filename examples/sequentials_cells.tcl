define_cell \
	-input { D } \
	-output { Q } \
	-clock { CLK } \
	-pinlist { CLK D Q } \
	-delay dly_10x10 \
	-power pwr_10x10 \
	-constraint con_10x10 \
	{ DFF }

define_cell \
	-input { D } \
	-output { Q } \
	-clock { CLK } \
	-pinlist { CLK D Q } \
	{ DFF_NOCOLLAPSE }

define_cell \
	-input { CLK D } \
	-output { Q } \
	-pinlist { CLK D Q } \
	{ UCDFF }

define_cell \
	-input { D } \
	-output { M Q } \
	-clock { CLK } \
	-pinlist { CLK D M Q } \
	{ EMDFF }

define_cell \
	-input { D } \
	-output { Q T } \
	-clock { CLK } \
	-pinlist { CLK D Q T } \
	{ TAPDFF }

define_cell \
	-input { D } \
	-output { Q } \
	-clock { CLK } \
	-pinlist { CLK D Q } \
	{ IDFF }

define_cell \
	-input { D } \
	-output { Q Qn } \
	-clock { CLK } \
	-pinlist { CLK D Q Qn } \
	{ XN }

define_cell \
	-output { Q } \
	-clock { CLK } \
	-async { R } \
	-pinlist { CLK R Q } \
	{ TFF }

define_cell \
	-input { D } \
	-output { Q } \
	-clock { CLK } \
	-pinlist { CLK D Q } \
	{ DET }

define_cell \
	-input { D R } \
	-output { Q } \
	-clock { CLK } \
	-pinlist { CLK D R Q } \
	{ MOR }

define_cell \
	-input { D } \
	-output { Q } \
	-clock { CLK } \
	-async { R } \
	-pinlist { CLK D R Q } \
	{ MORA }

define_cell \
	-input { D } \
	-output { Q } \
	-clock { CLK } \
	-async { R } \
	-pinlist { CLK D R Q } \
	{ BR }

define_cell \
	-input { D R } \
	-output { Q } \
	-clock { CLK } \
	-pinlist { CLK D R Q } \
	{ SYNCR }

define_cell \
	-input { D R G } \
	-output { Q } \
	-clock { CLK } \
	-pinlist { CLK D R G Q } \
	{ SYNCRG }

define_cell \
	-input { D R G } \
	-output { Q } \
	-clock { CLK } \
	-pinlist { CLK D R G Q } \
	{ GATEDR }

define_cell \
	-input { D } \
	-output { Q } \
	-clock { CLK } \
	-async { R G } \
	-pinlist { CLK D R G Q } \
	{ AGATEDR }

define_cell \
	-input { D } \
	-output { Q } \
	-clock { CLK R } \
	-pinlist { CLK D R Q } \
	{ RDFF }

define_cell \
	-input { D B } \
	-output { Q } \
	-clock { CLK } \
	-async { R } \
	-pinlist { CLK D B R Q } \
	{ COEX }

define_cell \
	-input { D } \
	-output { Q } \
	-clock { CLK } \
	-async { PRE CLR } \
	-pinlist { CLK D PRE CLR Q } \
	{ CAFF }

define_cell \
	-input { D } \
	-output { Q } \
	-clock { CLK } \
	-pinlist { CLK D Q } \
	{ DLAT }

define_cell \
	-input { D } \
	-output { Q } \
	-clock { EN } \
	-pinlist { EN D Q } \
	{ DLAT_EN }

define_cell \
	-input { E D } \
	-output { Q } \
	-pinlist { E D Q } \
	{ DLAT_E }

define_cell \
	-input { D } \
	-output { Q } \
	-clock { CLK } \
	-pinlist { CLK D Q } \
	{ GLAT }

define_cell \
	-input { D } \
	-output { Q } \
	-clock { CLKA CLKB } \
	-pinlist { CLKA CLKB D Q } \
	{ MUXLAT }

define_cell \
	-input { D } \
	-output { Q } \
	-clock { CLKA CLKB } \
	-pinlist { CLKA CLKB D Q } \
	{ MCDFF }

define_cell \
	-input { D } \
	-output { Q } \
	-clock { CLKA CLKB } \
	-pinlist { CLKA CLKB D Q } \
	{ MCDFFX1 MCDFFX4 }

define_cell \
	-input { D } \
	-output { Q } \
	-clock { CLK } \
	-pinlist { CLK D Q } \
	{ TCASC }

define_cell \
	-input { D } \
	-output { T } \
	-clock { CLK } \
	-pinlist { CLK D T } \
	{ XLAT }

define_cell \
	-input { D } \
	-output { Q } \
	-clock { CLKA CLKB } \
	-pinlist { CLKA CLKB D Q } \
	{ HPIPE }

define_cell \
	-input { DA DB } \
	-output { Q } \
	-clock { CLKA CLKB } \
	-pinlist { CLKA CLKB DA DB Q } \
	{ DCMUX }

define_cell \
	-input { EN } \
	-output { GCLK } \
	-clock { CLK } \
	-pinlist { CLK EN GCLK } \
	{ ICG }

define_cell \
	-input { RA RB S } \
	-output { GCLK } \
	-clock { CLKA CLKB } \
	-pinlist { CLKA CLKB RA RB S GCLK } \
	{ ICM }

define_cell \
	-input { C D } \
	-output { Y } \
	-pinlist { C D Y } \
	{ GL }

define_cell \
	-input { A B C D } \
	-output { Y Z } \
	-pinlist { A B C D Y Z } \
	{ MIX }

define_cell \
	-input { C D E } \
	-output { Z2 } \
	-pinlist { C D E Z2 } \
	{ TRW }

define_cell \
	-input { A Q_st } \
	-output { Q } \
	-pinlist { A Q_st Q } \
	{ COLL }

define_cell \
	-input { A B } \
	-output { Q Qc Qn } \
	-pinlist { A B Q Qc Qn } \
	{ C2P }

define_cell \
	-input { A B } \
	-output { Q } \
	-async { R } \
	-pinlist { A B R Q } \
	{ RC2 }

