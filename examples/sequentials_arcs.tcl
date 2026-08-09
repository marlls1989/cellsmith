define_arc \
	-type edge \
	-pinlist {CLK D Q} \
	-ic "0 0 $VDD" \
	-vector {R 0 F} \
	-related_pin CLK \
	-pin Q \
	{ DFF }

define_arc \
	-type edge \
	-pinlist {CLK D Q} \
	-ic "0 $VDD 0" \
	-vector {R 1 R} \
	-related_pin CLK \
	-pin Q \
	{ DFF }

define_arc \
	-type hidden \
	-pinlist {CLK D Q} \
	-ic "$VDD 0 0" \
	-vector {F 0 0} \
	-pin CLK \
	{ DFF }

define_arc \
	-type hidden \
	-pinlist {CLK D Q} \
	-ic "$VDD 0 0" \
	-vector {1 R 0} \
	-pin D \
	{ DFF }

define_arc \
	-type hidden \
	-pinlist {CLK D Q} \
	-ic "$VDD $VDD $VDD" \
	-vector {1 F 1} \
	-pin D \
	{ DFF }

define_arc \
	-type hidden \
	-pinlist {CLK D Q} \
	-ic "0 $VDD $VDD" \
	-vector {R 1 1} \
	-pin CLK \
	{ DFF }

define_leakage \
	-pinlist {CLK D Q} \
	-vector {1 0 0} \
	-when "CLK*!D*!Q" \
	{ DFF }

define_leakage \
	-pinlist {CLK D Q} \
	-vector {1 1 1} \
	-when "CLK*D*Q" \
	{ DFF }

define_leakage \
	-pinlist {CLK D Q} \
	-vector {0 1 1} \
	-when "!CLK*D*Q" \
	{ DFF }

define_leakage \
	-pinlist {CLK D Q} \
	-vector {0 0 0} \
	-when "!CLK*!D*!Q" \
	{ DFF }

define_leakage \
	-pinlist {CLK D Q} \
	-vector {1 1 0} \
	-when "CLK*D*!Q" \
	{ DFF }

define_leakage \
	-pinlist {CLK D Q} \
	-vector {1 0 1} \
	-when "CLK*!D*Q" \
	{ DFF }

define_leakage \
	-pinlist {CLK D Q} \
	-vector {0 0 1} \
	-when "!CLK*!D*Q" \
	{ DFF }

define_leakage \
	-pinlist {CLK D Q} \
	-vector {0 1 0} \
	-when "!CLK*D*!Q" \
	{ DFF }

define_arc \
	-type setup \
	-pinlist {CLK D M Q} \
	-ic "0 $VDD $VDD $VDD" \
	-vector {R F X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ DFF }

define_arc \
	-type hold \
	-pinlist {CLK D M Q} \
	-ic "0 $VDD $VDD $VDD" \
	-vector {R F X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ DFF }

define_arc \
	-type setup \
	-pinlist {CLK D M Q} \
	-ic "0 0 0 0" \
	-vector {R R X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ DFF }

define_arc \
	-type hold \
	-pinlist {CLK D M Q} \
	-ic "0 0 0 0" \
	-vector {R R X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ DFF }

define_arc \
	-type combinational \
	-pinlist {CLK D Q} \
	-ic "0 $VDD 0" \
	-vector {R 1 R} \
	-related_pin CLK \
	-pin Q \
	{ DFF_NOCOLLAPSE }

define_arc \
	-type combinational \
	-pinlist {CLK D Q} \
	-ic "0 0 $VDD" \
	-vector {R 0 F} \
	-related_pin CLK \
	-pin Q \
	{ DFF_NOCOLLAPSE }

define_arc \
	-type hidden \
	-pinlist {CLK D Q} \
	-ic "$VDD $VDD $VDD" \
	-vector {F 1 1} \
	-pin CLK \
	{ DFF_NOCOLLAPSE }

define_arc \
	-type hidden \
	-pinlist {CLK D Q} \
	-ic "$VDD $VDD $VDD" \
	-vector {1 F 1} \
	-pin D \
	{ DFF_NOCOLLAPSE }

define_arc \
	-type hidden \
	-pinlist {CLK D Q} \
	-ic "$VDD 0 0" \
	-vector {1 R 0} \
	-pin D \
	{ DFF_NOCOLLAPSE }

define_arc \
	-type hidden \
	-pinlist {CLK D Q} \
	-ic "0 $VDD $VDD" \
	-vector {R 1 1} \
	-pin CLK \
	{ DFF_NOCOLLAPSE }

define_leakage \
	-pinlist {CLK D Q} \
	-vector {1 1 1} \
	-when "CLK*D*Q" \
	{ DFF_NOCOLLAPSE }

define_leakage \
	-pinlist {CLK D Q} \
	-vector {1 0 0} \
	-when "CLK*!D*!Q" \
	{ DFF_NOCOLLAPSE }

define_leakage \
	-pinlist {CLK D Q} \
	-vector {1 0 1} \
	-when "CLK*!D*Q" \
	{ DFF_NOCOLLAPSE }

define_leakage \
	-pinlist {CLK D Q} \
	-vector {0 1 1} \
	-when "!CLK*D*Q" \
	{ DFF_NOCOLLAPSE }

define_leakage \
	-pinlist {CLK D Q} \
	-vector {0 0 0} \
	-when "!CLK*!D*!Q" \
	{ DFF_NOCOLLAPSE }

define_leakage \
	-pinlist {CLK D Q} \
	-vector {1 1 0} \
	-when "CLK*D*!Q" \
	{ DFF_NOCOLLAPSE }

define_leakage \
	-pinlist {CLK D Q} \
	-vector {0 1 0} \
	-when "!CLK*D*!Q" \
	{ DFF_NOCOLLAPSE }

define_leakage \
	-pinlist {CLK D Q} \
	-vector {0 0 1} \
	-when "!CLK*!D*Q" \
	{ DFF_NOCOLLAPSE }

define_arc \
	-type setup \
	-pinlist {CLK D M Q} \
	-ic "0 $VDD $VDD $VDD" \
	-vector {R F X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ DFF_NOCOLLAPSE }

define_arc \
	-type hold \
	-pinlist {CLK D M Q} \
	-ic "0 $VDD $VDD $VDD" \
	-vector {R F X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ DFF_NOCOLLAPSE }

define_arc \
	-type setup \
	-pinlist {CLK D M Q} \
	-ic "0 0 0 0" \
	-vector {R R X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ DFF_NOCOLLAPSE }

define_arc \
	-type hold \
	-pinlist {CLK D M Q} \
	-ic "0 0 0 0" \
	-vector {R R X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ DFF_NOCOLLAPSE }

define_arc \
	-type combinational \
	-pinlist {CLK D Q} \
	-ic "0 $VDD 0" \
	-vector {R 1 R} \
	-related_pin CLK \
	-pin Q \
	{ UCDFF }

define_arc \
	-type combinational \
	-pinlist {CLK D Q} \
	-ic "0 0 $VDD" \
	-vector {R 0 F} \
	-related_pin CLK \
	-pin Q \
	{ UCDFF }

define_arc \
	-type hidden \
	-pinlist {CLK D Q} \
	-ic "$VDD 0 0" \
	-vector {F 0 0} \
	-pin CLK \
	{ UCDFF }

define_arc \
	-type hidden \
	-pinlist {CLK D Q} \
	-ic "$VDD 0 0" \
	-vector {1 R 0} \
	-pin D \
	{ UCDFF }

define_arc \
	-type hidden \
	-pinlist {CLK D Q} \
	-ic "$VDD $VDD $VDD" \
	-vector {1 F 1} \
	-pin D \
	{ UCDFF }

define_arc \
	-type hidden \
	-pinlist {CLK D Q} \
	-ic "0 0 0" \
	-vector {R 0 0} \
	-pin CLK \
	{ UCDFF }

define_leakage \
	-pinlist {CLK D Q} \
	-vector {1 0 0} \
	-when "CLK*!D*!Q" \
	{ UCDFF }

define_leakage \
	-pinlist {CLK D Q} \
	-vector {1 1 1} \
	-when "CLK*D*Q" \
	{ UCDFF }

define_leakage \
	-pinlist {CLK D Q} \
	-vector {0 0 0} \
	-when "!CLK*!D*!Q" \
	{ UCDFF }

define_leakage \
	-pinlist {CLK D Q} \
	-vector {1 1 0} \
	-when "CLK*D*!Q" \
	{ UCDFF }

define_leakage \
	-pinlist {CLK D Q} \
	-vector {0 1 1} \
	-when "!CLK*D*Q" \
	{ UCDFF }

define_leakage \
	-pinlist {CLK D Q} \
	-vector {1 0 1} \
	-when "CLK*!D*Q" \
	{ UCDFF }

define_leakage \
	-pinlist {CLK D Q} \
	-vector {0 1 0} \
	-when "!CLK*D*!Q" \
	{ UCDFF }

define_leakage \
	-pinlist {CLK D Q} \
	-vector {0 0 1} \
	-when "!CLK*!D*Q" \
	{ UCDFF }

define_arc \
	-type non_seq_setup \
	-pinlist {CLK D M Q} \
	-ic "0 $VDD $VDD $VDD" \
	-vector {R F X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ UCDFF }

define_arc \
	-type non_seq_hold \
	-pinlist {CLK D M Q} \
	-ic "0 $VDD $VDD $VDD" \
	-vector {R F X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ UCDFF }

define_arc \
	-type non_seq_setup \
	-pinlist {CLK D M Q} \
	-ic "0 0 0 0" \
	-vector {R R X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ UCDFF }

define_arc \
	-type non_seq_hold \
	-pinlist {CLK D M Q} \
	-ic "0 0 0 0" \
	-vector {R R X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ UCDFF }

define_arc \
	-type edge \
	-pinlist {CLK D M Q} \
	-ic "$VDD 0 $VDD $VDD" \
	-vector {F 0 F X} \
	-related_pin CLK \
	-pin M \
	{ EMDFF }

define_arc \
	-type combinational \
	-pinlist {CLK D M Q} \
	-ic "0 $VDD $VDD $VDD" \
	-vector {0 F F X} \
	-related_pin D \
	-pin M \
	{ EMDFF }

define_arc \
	-type combinational \
	-pinlist {CLK D M Q} \
	-ic "0 0 0 0" \
	-vector {0 R R X} \
	-related_pin D \
	-pin M \
	{ EMDFF }

define_arc \
	-type edge \
	-pinlist {CLK D M Q} \
	-ic "$VDD $VDD 0 0" \
	-vector {F 1 R X} \
	-related_pin CLK \
	-pin M \
	{ EMDFF }

define_arc \
	-type edge \
	-pinlist {CLK D M Q} \
	-ic "0 0 0 $VDD" \
	-vector {R 0 X F} \
	-related_pin CLK \
	-pin Q \
	{ EMDFF }

define_arc \
	-type edge \
	-pinlist {CLK D M Q} \
	-ic "0 $VDD $VDD 0" \
	-vector {R 1 X R} \
	-related_pin CLK \
	-pin Q \
	{ EMDFF }

define_arc \
	-type hidden \
	-pinlist {CLK D M Q} \
	-ic "$VDD $VDD $VDD $VDD" \
	-vector {F 1 1 1} \
	-pin CLK \
	{ EMDFF }

define_arc \
	-type hidden \
	-pinlist {CLK D M Q} \
	-ic "$VDD $VDD $VDD $VDD" \
	-vector {1 F 1 1} \
	-pin D \
	{ EMDFF }

define_arc \
	-type hidden \
	-pinlist {CLK D M Q} \
	-ic "$VDD 0 0 0" \
	-vector {1 R 0 0} \
	-pin D \
	{ EMDFF }

define_arc \
	-type hidden \
	-pinlist {CLK D M Q} \
	-ic "0 $VDD $VDD $VDD" \
	-vector {R 1 1 1} \
	-pin CLK \
	{ EMDFF }

define_leakage \
	-pinlist {CLK D M Q} \
	-vector {1 1 1 1} \
	-when "CLK*D*M*Q" \
	{ EMDFF }

define_leakage \
	-pinlist {CLK D M Q} \
	-vector {1 0 0 0} \
	-when "CLK*!D*!M*!Q" \
	{ EMDFF }

define_leakage \
	-pinlist {CLK D M Q} \
	-vector {1 0 1 1} \
	-when "CLK*!D*M*Q" \
	{ EMDFF }

define_leakage \
	-pinlist {CLK D M Q} \
	-vector {0 1 1 1} \
	-when "!CLK*D*M*Q" \
	{ EMDFF }

define_leakage \
	-pinlist {CLK D M Q} \
	-vector {0 0 0 0} \
	-when "!CLK*!D*!M*!Q" \
	{ EMDFF }

define_leakage \
	-pinlist {CLK D M Q} \
	-vector {1 1 0 0} \
	-when "CLK*D*!M*!Q" \
	{ EMDFF }

define_leakage \
	-pinlist {CLK D M Q} \
	-vector {0 0 0 1} \
	-when "!CLK*!D*!M*Q" \
	{ EMDFF }

define_leakage \
	-pinlist {CLK D M Q} \
	-vector {0 1 1 0} \
	-when "!CLK*D*M*!Q" \
	{ EMDFF }

define_arc \
	-type setup \
	-pinlist {CLK D M Q} \
	-ic "0 $VDD $VDD $VDD" \
	-vector {R F X X} \
	-related_pin CLK \
	-pin D \
	-probe {M Q} \
	{ EMDFF }

define_arc \
	-type hold \
	-pinlist {CLK D M Q} \
	-ic "0 $VDD $VDD $VDD" \
	-vector {R F X X} \
	-related_pin CLK \
	-pin D \
	-probe {M Q} \
	{ EMDFF }

define_arc \
	-type setup \
	-pinlist {CLK D M Q} \
	-ic "0 0 0 0" \
	-vector {R R X X} \
	-related_pin CLK \
	-pin D \
	-probe {M Q} \
	{ EMDFF }

define_arc \
	-type hold \
	-pinlist {CLK D M Q} \
	-ic "0 0 0 0" \
	-vector {R R X X} \
	-related_pin CLK \
	-pin D \
	-probe {M Q} \
	{ EMDFF }

define_arc \
	-type edge \
	-pinlist {CLK D Q T} \
	-ic "$VDD $VDD 0 0" \
	-vector {F 1 X R} \
	-related_pin CLK \
	-pin T \
	{ TAPDFF }

define_arc \
	-type edge \
	-pinlist {CLK D Q T} \
	-ic "$VDD 0 $VDD $VDD" \
	-vector {F 0 X F} \
	-related_pin CLK \
	-pin T \
	{ TAPDFF }

define_arc \
	-type combinational \
	-pinlist {CLK D Q T} \
	-ic "0 0 0 0" \
	-vector {0 R X R} \
	-related_pin D \
	-pin T \
	{ TAPDFF }

define_arc \
	-type combinational \
	-pinlist {CLK D Q T} \
	-ic "0 $VDD $VDD $VDD" \
	-vector {0 F X F} \
	-related_pin D \
	-pin T \
	{ TAPDFF }

define_arc \
	-type edge \
	-pinlist {CLK D Q T} \
	-ic "0 $VDD 0 $VDD" \
	-vector {R 1 R X} \
	-related_pin CLK \
	-pin Q \
	{ TAPDFF }

define_arc \
	-type edge \
	-pinlist {CLK D Q T} \
	-ic "0 0 $VDD 0" \
	-vector {R 0 F X} \
	-related_pin CLK \
	-pin Q \
	{ TAPDFF }

define_arc \
	-type hidden \
	-pinlist {CLK D Q T} \
	-ic "$VDD $VDD $VDD $VDD" \
	-vector {F 1 1 1} \
	-pin CLK \
	{ TAPDFF }

define_arc \
	-type hidden \
	-pinlist {CLK D Q T} \
	-ic "$VDD $VDD $VDD $VDD" \
	-vector {1 F 1 1} \
	-pin D \
	{ TAPDFF }

define_arc \
	-type hidden \
	-pinlist {CLK D Q T} \
	-ic "$VDD 0 0 0" \
	-vector {1 R 0 0} \
	-pin D \
	{ TAPDFF }

define_arc \
	-type hidden \
	-pinlist {CLK D Q T} \
	-ic "0 0 0 0" \
	-vector {R 0 0 0} \
	-pin CLK \
	{ TAPDFF }

define_leakage \
	-pinlist {CLK D Q T} \
	-vector {1 1 1 1} \
	-when "CLK*D*Q*T" \
	{ TAPDFF }

define_leakage \
	-pinlist {CLK D Q T} \
	-vector {1 0 0 0} \
	-when "CLK*!D*!Q*!T" \
	{ TAPDFF }

define_leakage \
	-pinlist {CLK D Q T} \
	-vector {1 1 0 0} \
	-when "CLK*D*!Q*!T" \
	{ TAPDFF }

define_leakage \
	-pinlist {CLK D Q T} \
	-vector {1 0 1 1} \
	-when "CLK*!D*Q*T" \
	{ TAPDFF }

define_leakage \
	-pinlist {CLK D Q T} \
	-vector {0 0 0 0} \
	-when "!CLK*!D*!Q*!T" \
	{ TAPDFF }

define_leakage \
	-pinlist {CLK D Q T} \
	-vector {0 1 1 1} \
	-when "!CLK*D*Q*T" \
	{ TAPDFF }

define_leakage \
	-pinlist {CLK D Q T} \
	-vector {0 1 0 1} \
	-when "!CLK*D*!Q*T" \
	{ TAPDFF }

define_leakage \
	-pinlist {CLK D Q T} \
	-vector {0 0 1 0} \
	-when "!CLK*!D*Q*!T" \
	{ TAPDFF }

define_arc \
	-type setup \
	-pinlist {CLK D Q T} \
	-ic "0 $VDD $VDD $VDD" \
	-vector {R F X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q T} \
	{ TAPDFF }

define_arc \
	-type hold \
	-pinlist {CLK D Q T} \
	-ic "0 $VDD $VDD $VDD" \
	-vector {R F X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q T} \
	{ TAPDFF }

define_arc \
	-type setup \
	-pinlist {CLK D Q T} \
	-ic "0 0 0 0" \
	-vector {R R X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q T} \
	{ TAPDFF }

define_arc \
	-type hold \
	-pinlist {CLK D Q T} \
	-ic "0 0 0 0" \
	-vector {R R X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q T} \
	{ TAPDFF }

define_arc \
	-type edge \
	-pinlist {CLK D Q} \
	-ic "0 0 0" \
	-vector {R 0 R} \
	-related_pin CLK \
	-pin Q \
	{ IDFF }

define_arc \
	-type edge \
	-pinlist {CLK D Q} \
	-ic "0 $VDD $VDD" \
	-vector {R 1 F} \
	-related_pin CLK \
	-pin Q \
	{ IDFF }

define_arc \
	-type hidden \
	-pinlist {CLK D Q} \
	-ic "$VDD 0 $VDD" \
	-vector {F 0 1} \
	-pin CLK \
	{ IDFF }

define_arc \
	-type hidden \
	-pinlist {CLK D Q} \
	-ic "$VDD 0 $VDD" \
	-vector {1 R 1} \
	-pin D \
	{ IDFF }

define_arc \
	-type hidden \
	-pinlist {CLK D Q} \
	-ic "$VDD $VDD 0" \
	-vector {1 F 0} \
	-pin D \
	{ IDFF }

define_arc \
	-type hidden \
	-pinlist {CLK D Q} \
	-ic "0 $VDD 0" \
	-vector {R 1 0} \
	-pin CLK \
	{ IDFF }

define_leakage \
	-pinlist {CLK D Q} \
	-vector {1 0 1} \
	-when "CLK*!D*Q" \
	{ IDFF }

define_leakage \
	-pinlist {CLK D Q} \
	-vector {1 1 0} \
	-when "CLK*D*!Q" \
	{ IDFF }

define_leakage \
	-pinlist {CLK D Q} \
	-vector {0 1 0} \
	-when "!CLK*D*!Q" \
	{ IDFF }

define_leakage \
	-pinlist {CLK D Q} \
	-vector {1 0 0} \
	-when "CLK*!D*!Q" \
	{ IDFF }

define_leakage \
	-pinlist {CLK D Q} \
	-vector {0 0 1} \
	-when "!CLK*!D*Q" \
	{ IDFF }

define_leakage \
	-pinlist {CLK D Q} \
	-vector {1 1 1} \
	-when "CLK*D*Q" \
	{ IDFF }

define_leakage \
	-pinlist {CLK D Q} \
	-vector {0 0 0} \
	-when "!CLK*!D*!Q" \
	{ IDFF }

define_leakage \
	-pinlist {CLK D Q} \
	-vector {0 1 1} \
	-when "!CLK*D*Q" \
	{ IDFF }

define_arc \
	-type setup \
	-pinlist {CLK D M Q} \
	-ic "0 $VDD $VDD 0" \
	-vector {R F X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ IDFF }

define_arc \
	-type hold \
	-pinlist {CLK D M Q} \
	-ic "0 $VDD $VDD 0" \
	-vector {R F X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ IDFF }

define_arc \
	-type setup \
	-pinlist {CLK D M Q} \
	-ic "0 0 0 $VDD" \
	-vector {R R X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ IDFF }

define_arc \
	-type hold \
	-pinlist {CLK D M Q} \
	-ic "0 0 0 $VDD" \
	-vector {R R X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ IDFF }

define_arc \
	-type edge \
	-pinlist {CLK D Q Qn} \
	-ic "0 0 $VDD 0" \
	-vector {R 0 F X} \
	-related_pin CLK \
	-pin Q \
	{ XN }

define_arc \
	-type edge \
	-pinlist {CLK D Q Qn} \
	-ic "0 0 $VDD 0" \
	-vector {R 0 X R} \
	-related_pin CLK \
	-pin Qn \
	{ XN }

define_arc \
	-type edge \
	-pinlist {CLK D Q Qn} \
	-ic "0 $VDD 0 $VDD" \
	-vector {R 1 R X} \
	-related_pin CLK \
	-pin Q \
	{ XN }

define_arc \
	-type edge \
	-pinlist {CLK D Q Qn} \
	-ic "0 $VDD 0 $VDD" \
	-vector {R 1 X F} \
	-related_pin CLK \
	-pin Qn \
	{ XN }

define_arc \
	-type hidden \
	-pinlist {CLK D Q Qn} \
	-ic "$VDD 0 0 $VDD" \
	-vector {F 0 0 1} \
	-pin CLK \
	{ XN }

define_arc \
	-type hidden \
	-pinlist {CLK D Q Qn} \
	-ic "$VDD 0 0 $VDD" \
	-vector {1 R 0 1} \
	-pin D \
	{ XN }

define_arc \
	-type hidden \
	-pinlist {CLK D Q Qn} \
	-ic "$VDD $VDD $VDD 0" \
	-vector {1 F 1 0} \
	-pin D \
	{ XN }

define_arc \
	-type hidden \
	-pinlist {CLK D Q Qn} \
	-ic "0 $VDD $VDD 0" \
	-vector {R 1 1 0} \
	-pin CLK \
	{ XN }

define_leakage \
	-pinlist {CLK D Q Qn} \
	-vector {1 0 0 1} \
	-when "CLK*!D*!Q*Qn" \
	{ XN }

define_leakage \
	-pinlist {CLK D Q Qn} \
	-vector {1 1 1 0} \
	-when "CLK*D*Q*!Qn" \
	{ XN }

define_leakage \
	-pinlist {CLK D Q Qn} \
	-vector {0 1 1 0} \
	-when "!CLK*D*Q*!Qn" \
	{ XN }

define_leakage \
	-pinlist {CLK D Q Qn} \
	-vector {1 1 0 1} \
	-when "CLK*D*!Q*Qn" \
	{ XN }

define_leakage \
	-pinlist {CLK D Q Qn} \
	-vector {1 0 1 0} \
	-when "CLK*!D*Q*!Qn" \
	{ XN }

define_leakage \
	-pinlist {CLK D Q Qn} \
	-vector {0 0 0 1} \
	-when "!CLK*!D*!Q*Qn" \
	{ XN }

define_leakage \
	-pinlist {CLK D Q Qn} \
	-vector {0 0 1 0} \
	-when "!CLK*!D*Q*!Qn" \
	{ XN }

define_leakage \
	-pinlist {CLK D Q Qn} \
	-vector {0 1 0 1} \
	-when "!CLK*D*!Q*Qn" \
	{ XN }

define_arc \
	-type setup \
	-pinlist {CLK D M Q Qn} \
	-ic "0 $VDD $VDD $VDD 0" \
	-vector {R F X X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q Qn M} \
	{ XN }

define_arc \
	-type hold \
	-pinlist {CLK D M Q Qn} \
	-ic "0 $VDD $VDD $VDD 0" \
	-vector {R F X X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q Qn M} \
	{ XN }

define_arc \
	-type setup \
	-pinlist {CLK D M Q Qn} \
	-ic "0 0 0 0 $VDD" \
	-vector {R R X X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q Qn M} \
	{ XN }

define_arc \
	-type hold \
	-pinlist {CLK D M Q Qn} \
	-ic "0 0 0 0 $VDD" \
	-vector {R R X X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q Qn M} \
	{ XN }

define_arc \
	-type edge \
	-pinlist {CLK R Q} \
	-ic "0 0 0" \
	-vector {R 0 R} \
	-related_pin CLK \
	-pin Q \
	{ TFF }

define_arc \
	-type async \
	-pinlist {CLK R Q} \
	-ic "$VDD 0 $VDD" \
	-vector {1 R F} \
	-related_pin R \
	-pin Q \
	{ TFF }

define_arc \
	-type edge \
	-pinlist {CLK R Q} \
	-ic "0 0 $VDD" \
	-vector {R 0 F} \
	-related_pin CLK \
	-pin Q \
	{ TFF }

define_arc \
	-type hidden \
	-pinlist {CLK R Q} \
	-ic "0 $VDD 0" \
	-vector {R 1 0} \
	-pin CLK \
	{ TFF }

define_arc \
	-type hidden \
	-pinlist {CLK R Q} \
	-ic "0 $VDD 0" \
	-vector {0 F 0} \
	-pin R \
	{ TFF }

define_arc \
	-type hidden \
	-pinlist {CLK R Q} \
	-ic "$VDD $VDD 0" \
	-vector {F 1 0} \
	-pin CLK \
	{ TFF }

define_arc \
	-type hidden \
	-pinlist {CLK R Q} \
	-ic "0 0 0" \
	-vector {0 R 0} \
	-pin R \
	{ TFF }

define_leakage -when "!CLK*!Q*R" { TFF }

define_leakage -when "CLK*!Q*R" { TFF }

define_leakage \
	-pinlist {CLK R Q} \
	-vector {0 0 0} \
	-when "!CLK*!Q*!R" \
	{ TFF }

define_leakage \
	-pinlist {CLK R Q} \
	-vector {1 0 0} \
	-when "CLK*!Q*!R" \
	{ TFF }

define_leakage \
	-pinlist {CLK R Q} \
	-vector {1 0 1} \
	-when "CLK*Q*!R" \
	{ TFF }

define_leakage \
	-pinlist {CLK R Q} \
	-vector {0 0 1} \
	-when "!CLK*Q*!R" \
	{ TFF }

define_arc \
	-type setup \
	-pinlist {CLK R M Q} \
	-ic "0 $VDD 0 0" \
	-vector {R F X X} \
	-related_pin CLK \
	-pin R \
	-probe {Q M} \
	{ TFF }

define_arc \
	-type hold \
	-pinlist {CLK R M Q} \
	-ic "0 $VDD 0 0" \
	-vector {R F X X} \
	-related_pin CLK \
	-pin R \
	-probe {Q M} \
	{ TFF }

define_arc \
	-type edge \
	-pinlist {CLK D Q} \
	-ic "$VDD 0 $VDD" \
	-vector {F 0 F} \
	-related_pin CLK \
	-pin Q \
	{ DET }

define_arc \
	-type edge \
	-pinlist {CLK D Q} \
	-ic "0 $VDD 0" \
	-vector {R 1 R} \
	-related_pin CLK \
	-pin Q \
	{ DET }

define_arc \
	-type edge \
	-pinlist {CLK D Q} \
	-ic "$VDD $VDD 0" \
	-vector {F 1 R} \
	-related_pin CLK \
	-pin Q \
	{ DET }

define_arc \
	-type edge \
	-pinlist {CLK D Q} \
	-ic "0 0 $VDD" \
	-vector {R 0 F} \
	-related_pin CLK \
	-pin Q \
	{ DET }

define_arc \
	-type hidden \
	-pinlist {CLK D Q} \
	-ic "$VDD $VDD $VDD" \
	-vector {F 1 1} \
	-pin CLK \
	{ DET }

define_arc \
	-type hidden \
	-pinlist {CLK D Q} \
	-ic "$VDD $VDD $VDD" \
	-vector {1 F 1} \
	-pin D \
	{ DET }

define_arc \
	-type hidden \
	-pinlist {CLK D Q} \
	-ic "0 0 0" \
	-vector {R 0 0} \
	-pin CLK \
	{ DET }

define_arc \
	-type hidden \
	-pinlist {CLK D Q} \
	-ic "0 0 0" \
	-vector {0 R 0} \
	-pin D \
	{ DET }

define_leakage \
	-pinlist {CLK D Q} \
	-vector {1 1 1} \
	-when "CLK*D*Q" \
	{ DET }

define_leakage \
	-pinlist {CLK D Q} \
	-vector {0 0 0} \
	-when "!CLK*!D*!Q" \
	{ DET }

define_leakage \
	-pinlist {CLK D Q} \
	-vector {1 0 0} \
	-when "CLK*!D*!Q" \
	{ DET }

define_leakage \
	-pinlist {CLK D Q} \
	-vector {0 1 1} \
	-when "!CLK*D*Q" \
	{ DET }

define_leakage \
	-pinlist {CLK D Q} \
	-vector {1 0 1} \
	-when "CLK*!D*Q" \
	{ DET }

define_leakage \
	-pinlist {CLK D Q} \
	-vector {0 1 0} \
	-when "!CLK*D*!Q" \
	{ DET }

define_leakage \
	-pinlist {CLK D Q} \
	-vector {1 1 0} \
	-when "CLK*D*!Q" \
	{ DET }

define_leakage \
	-pinlist {CLK D Q} \
	-vector {0 0 1} \
	-when "!CLK*!D*Q" \
	{ DET }

define_arc \
	-type setup \
	-pinlist {CLK D L2 Q} \
	-ic "$VDD $VDD $VDD $VDD" \
	-vector {F F X X} \
	-related_pin CLK \
	-pin D \
	-probe {L2} \
	{ DET }

define_arc \
	-type hold \
	-pinlist {CLK D L2 Q} \
	-ic "$VDD $VDD $VDD $VDD" \
	-vector {F F X X} \
	-related_pin CLK \
	-pin D \
	-probe {L2} \
	{ DET }

define_arc \
	-type setup \
	-pinlist {CLK D L2 Q} \
	-ic "$VDD 0 0 0" \
	-vector {F R X X} \
	-related_pin CLK \
	-pin D \
	-probe {L2} \
	{ DET }

define_arc \
	-type hold \
	-pinlist {CLK D L2 Q} \
	-ic "$VDD 0 0 0" \
	-vector {F R X X} \
	-related_pin CLK \
	-pin D \
	-probe {L2} \
	{ DET }

define_arc \
	-type setup \
	-pinlist {CLK D L1 Q} \
	-ic "0 $VDD $VDD $VDD" \
	-vector {R F X X} \
	-related_pin CLK \
	-pin D \
	-probe {L1} \
	{ DET }

define_arc \
	-type hold \
	-pinlist {CLK D L1 Q} \
	-ic "0 $VDD $VDD $VDD" \
	-vector {R F X X} \
	-related_pin CLK \
	-pin D \
	-probe {L1} \
	{ DET }

define_arc \
	-type setup \
	-pinlist {CLK D L1 Q} \
	-ic "0 0 0 0" \
	-vector {R R X X} \
	-related_pin CLK \
	-pin D \
	-probe {L1} \
	{ DET }

define_arc \
	-type hold \
	-pinlist {CLK D L1 Q} \
	-ic "0 0 0 0" \
	-vector {R R X X} \
	-related_pin CLK \
	-pin D \
	-probe {L1} \
	{ DET }

define_arc \
	-type combinational \
	-pinlist {CLK D R Q} \
	-ic "$VDD $VDD 0 $VDD" \
	-vector {1 1 R F} \
	-related_pin R \
	-pin Q \
	{ MOR }

define_arc \
	-type edge \
	-pinlist {CLK D R Q} \
	-ic "0 $VDD 0 0" \
	-vector {R 1 0 R} \
	-related_pin CLK \
	-pin Q \
	{ MOR }

define_arc \
	-type edge \
	-pinlist {CLK D R Q} \
	-ic "0 $VDD $VDD $VDD" \
	-vector {R 1 1 F} \
	-related_pin CLK \
	-pin Q \
	{ MOR }

define_arc \
	-type hidden \
	-pinlist {CLK D R Q} \
	-ic "$VDD 0 $VDD 0" \
	-vector {F 0 1 0} \
	-pin CLK \
	{ MOR }

define_arc \
	-type hidden \
	-pinlist {CLK D R Q} \
	-ic "$VDD 0 $VDD 0" \
	-vector {1 R 1 0} \
	-pin D \
	{ MOR }

define_arc \
	-type hidden \
	-pinlist {CLK D R Q} \
	-ic "$VDD 0 $VDD 0" \
	-vector {1 0 F 0} \
	-pin R \
	{ MOR }

define_arc \
	-type hidden \
	-pinlist {CLK D R Q} \
	-ic "$VDD $VDD $VDD 0" \
	-vector {1 F 1 0} \
	-pin D \
	{ MOR }

define_arc \
	-type hidden \
	-pinlist {CLK D R Q} \
	-ic "0 0 $VDD 0" \
	-vector {R 0 1 0} \
	-pin CLK \
	{ MOR }

define_arc \
	-type hidden \
	-pinlist {CLK D R Q} \
	-ic "$VDD $VDD 0 0" \
	-vector {1 1 R 0} \
	-pin R \
	{ MOR }

define_leakage -when "CLK*!D*!Q*R" { MOR }

define_leakage -when "CLK*D*!Q*R" { MOR }

define_leakage \
	-pinlist {CLK D R Q} \
	-vector {1 1 0 1} \
	-when "CLK*D*Q*!R" \
	{ MOR }

define_leakage \
	-pinlist {CLK D R Q} \
	-vector {0 0 1 0} \
	-when "!CLK*!D*!Q*R" \
	{ MOR }

define_leakage \
	-pinlist {CLK D R Q} \
	-vector {0 1 1 0} \
	-when "!CLK*D*!Q*R" \
	{ MOR }

define_leakage \
	-pinlist {CLK D R Q} \
	-vector {1 1 0 0} \
	-when "CLK*D*!Q*!R" \
	{ MOR }

define_leakage \
	-pinlist {CLK D R Q} \
	-vector {1 0 0 0} \
	-when "CLK*!D*!Q*!R" \
	{ MOR }

define_leakage \
	-pinlist {CLK D R Q} \
	-vector {0 1 0 0} \
	-when "!CLK*D*!Q*!R" \
	{ MOR }

define_leakage \
	-pinlist {CLK D R Q} \
	-vector {1 0 0 1} \
	-when "CLK*!D*Q*!R" \
	{ MOR }

define_leakage \
	-pinlist {CLK D R Q} \
	-vector {0 1 0 1} \
	-when "!CLK*D*Q*!R" \
	{ MOR }

define_leakage \
	-pinlist {CLK D R Q} \
	-vector {0 0 0 0} \
	-when "!CLK*!D*!Q*!R" \
	{ MOR }

define_leakage \
	-pinlist {CLK D R Q} \
	-vector {0 1 1 1} \
	-when "!CLK*D*Q*R" \
	{ MOR }

define_leakage \
	-pinlist {CLK D R Q} \
	-vector {0 0 0 1} \
	-when "!CLK*!D*Q*!R" \
	{ MOR }

define_leakage \
	-pinlist {CLK D R Q} \
	-vector {0 0 1 1} \
	-when "!CLK*!D*Q*R" \
	{ MOR }

define_arc \
	-type setup \
	-pinlist {CLK D R M Q} \
	-ic "0 $VDD 0 $VDD 0" \
	-vector {R F 0 X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ MOR }

define_arc \
	-type hold \
	-pinlist {CLK D R M Q} \
	-ic "0 $VDD 0 $VDD 0" \
	-vector {R F 0 X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ MOR }

define_arc \
	-type setup \
	-pinlist {CLK D R M Q} \
	-ic "0 0 0 0 0" \
	-vector {R R 0 X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ MOR }

define_arc \
	-type hold \
	-pinlist {CLK D R M Q} \
	-ic "0 0 0 0 0" \
	-vector {R R 0 X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ MOR }

define_arc \
	-type setup \
	-pinlist {CLK D R M Q} \
	-ic "0 $VDD $VDD 0 0" \
	-vector {R 1 F X X} \
	-related_pin CLK \
	-pin R \
	-probe {Q M} \
	{ MOR }

define_arc \
	-type hold \
	-pinlist {CLK D R M Q} \
	-ic "0 $VDD $VDD 0 0" \
	-vector {R 1 F X X} \
	-related_pin CLK \
	-pin R \
	-probe {Q M} \
	{ MOR }

define_arc \
	-type async \
	-pinlist {CLK D R Q} \
	-ic "$VDD $VDD 0 $VDD" \
	-vector {1 1 R F} \
	-related_pin R \
	-pin Q \
	{ MORA }

define_arc \
	-type edge \
	-pinlist {CLK D R Q} \
	-ic "0 $VDD 0 0" \
	-vector {R 1 0 R} \
	-related_pin CLK \
	-pin Q \
	{ MORA }

define_arc \
	-type edge \
	-pinlist {CLK D R Q} \
	-ic "0 $VDD $VDD $VDD" \
	-vector {R 1 1 F} \
	-related_pin CLK \
	-pin Q \
	{ MORA }

define_arc \
	-type hidden \
	-pinlist {CLK D R Q} \
	-ic "$VDD 0 $VDD 0" \
	-vector {F 0 1 0} \
	-pin CLK \
	{ MORA }

define_arc \
	-type hidden \
	-pinlist {CLK D R Q} \
	-ic "$VDD 0 $VDD 0" \
	-vector {1 R 1 0} \
	-pin D \
	{ MORA }

define_arc \
	-type hidden \
	-pinlist {CLK D R Q} \
	-ic "$VDD 0 $VDD 0" \
	-vector {1 0 F 0} \
	-pin R \
	{ MORA }

define_arc \
	-type hidden \
	-pinlist {CLK D R Q} \
	-ic "$VDD $VDD $VDD 0" \
	-vector {1 F 1 0} \
	-pin D \
	{ MORA }

define_arc \
	-type hidden \
	-pinlist {CLK D R Q} \
	-ic "0 $VDD $VDD 0" \
	-vector {R 1 1 0} \
	-pin CLK \
	{ MORA }

define_arc \
	-type hidden \
	-pinlist {CLK D R Q} \
	-ic "$VDD $VDD 0 0" \
	-vector {1 1 R 0} \
	-pin R \
	{ MORA }

define_leakage -when "CLK*!D*!Q*R" { MORA }

define_leakage -when "CLK*D*!Q*R" { MORA }

define_leakage \
	-pinlist {CLK D R Q} \
	-vector {0 1 1 0} \
	-when "!CLK*D*!Q*R" \
	{ MORA }

define_leakage \
	-pinlist {CLK D R Q} \
	-vector {1 1 0 0} \
	-when "CLK*D*!Q*!R" \
	{ MORA }

define_leakage \
	-pinlist {CLK D R Q} \
	-vector {1 0 0 0} \
	-when "CLK*!D*!Q*!R" \
	{ MORA }

define_leakage \
	-pinlist {CLK D R Q} \
	-vector {1 1 0 1} \
	-when "CLK*D*Q*!R" \
	{ MORA }

define_leakage \
	-pinlist {CLK D R Q} \
	-vector {0 0 1 0} \
	-when "!CLK*!D*!Q*R" \
	{ MORA }

define_leakage \
	-pinlist {CLK D R Q} \
	-vector {0 1 0 0} \
	-when "!CLK*D*!Q*!R" \
	{ MORA }

define_leakage \
	-pinlist {CLK D R Q} \
	-vector {1 0 0 1} \
	-when "CLK*!D*Q*!R" \
	{ MORA }

define_leakage \
	-pinlist {CLK D R Q} \
	-vector {0 0 0 0} \
	-when "!CLK*!D*!Q*!R" \
	{ MORA }

define_leakage \
	-pinlist {CLK D R Q} \
	-vector {0 1 0 1} \
	-when "!CLK*D*Q*!R" \
	{ MORA }

define_leakage \
	-pinlist {CLK D R Q} \
	-vector {0 1 1 1} \
	-when "!CLK*D*Q*R" \
	{ MORA }

define_leakage \
	-pinlist {CLK D R Q} \
	-vector {0 0 0 1} \
	-when "!CLK*!D*Q*!R" \
	{ MORA }

define_leakage \
	-pinlist {CLK D R Q} \
	-vector {0 0 1 1} \
	-when "!CLK*!D*Q*R" \
	{ MORA }

define_arc \
	-type setup \
	-pinlist {CLK D R M Q} \
	-ic "0 $VDD 0 $VDD 0" \
	-vector {R F 0 X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ MORA }

define_arc \
	-type hold \
	-pinlist {CLK D R M Q} \
	-ic "0 $VDD 0 $VDD 0" \
	-vector {R F 0 X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ MORA }

define_arc \
	-type setup \
	-pinlist {CLK D R M Q} \
	-ic "0 0 0 0 0" \
	-vector {R R 0 X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ MORA }

define_arc \
	-type hold \
	-pinlist {CLK D R M Q} \
	-ic "0 0 0 0 0" \
	-vector {R R 0 X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ MORA }

define_arc \
	-type setup \
	-pinlist {CLK D R M Q} \
	-ic "0 $VDD $VDD 0 0" \
	-vector {R 1 F X X} \
	-related_pin CLK \
	-pin R \
	-probe {Q M} \
	{ MORA }

define_arc \
	-type hold \
	-pinlist {CLK D R M Q} \
	-ic "0 $VDD $VDD 0 0" \
	-vector {R 1 F X X} \
	-related_pin CLK \
	-pin R \
	-probe {Q M} \
	{ MORA }

define_arc \
	-type edge \
	-pinlist {CLK D R Q} \
	-ic "0 $VDD 0 0" \
	-vector {R 1 0 R} \
	-related_pin CLK \
	-pin Q \
	{ BR }

define_arc \
	-type async \
	-pinlist {CLK D R Q} \
	-ic "$VDD $VDD 0 $VDD" \
	-vector {1 1 R F} \
	-related_pin R \
	-pin Q \
	{ BR }

define_arc \
	-type edge \
	-pinlist {CLK D R Q} \
	-ic "0 0 0 $VDD" \
	-vector {R 0 0 F} \
	-related_pin CLK \
	-pin Q \
	{ BR }

define_arc \
	-type hidden \
	-pinlist {CLK D R Q} \
	-ic "0 0 $VDD 0" \
	-vector {R 0 1 0} \
	-pin CLK \
	{ BR }

define_arc \
	-type hidden \
	-pinlist {CLK D R Q} \
	-ic "0 0 $VDD 0" \
	-vector {0 R 1 0} \
	-pin D \
	{ BR }

define_arc \
	-type hidden \
	-pinlist {CLK D R Q} \
	-ic "0 0 $VDD 0" \
	-vector {0 0 F 0} \
	-pin R \
	{ BR }

define_arc \
	-type hidden \
	-pinlist {CLK D R Q} \
	-ic "0 $VDD $VDD 0" \
	-vector {0 F 1 0} \
	-pin D \
	{ BR }

define_arc \
	-type hidden \
	-pinlist {CLK D R Q} \
	-ic "$VDD 0 $VDD 0" \
	-vector {F 0 1 0} \
	-pin CLK \
	{ BR }

define_arc \
	-type hidden \
	-pinlist {CLK D R Q} \
	-ic "0 0 0 0" \
	-vector {0 0 R 0} \
	-pin R \
	{ BR }

define_leakage -when "!CLK*!D*!Q*R" { BR }

define_leakage -when "!CLK*D*!Q*R" { BR }

define_leakage -when "CLK*!D*!Q*R" { BR }

define_leakage -when "CLK*D*!Q*R" { BR }

define_leakage \
	-pinlist {CLK D R Q} \
	-vector {0 0 0 0} \
	-when "!CLK*!D*!Q*!R" \
	{ BR }

define_leakage \
	-pinlist {CLK D R Q} \
	-vector {0 1 0 0} \
	-when "!CLK*D*!Q*!R" \
	{ BR }

define_leakage \
	-pinlist {CLK D R Q} \
	-vector {1 0 0 0} \
	-when "CLK*!D*!Q*!R" \
	{ BR }

define_leakage \
	-pinlist {CLK D R Q} \
	-vector {1 1 0 1} \
	-when "CLK*D*Q*!R" \
	{ BR }

define_leakage \
	-pinlist {CLK D R Q} \
	-vector {1 1 0 0} \
	-when "CLK*D*!Q*!R" \
	{ BR }

define_leakage \
	-pinlist {CLK D R Q} \
	-vector {0 1 0 1} \
	-when "!CLK*D*Q*!R" \
	{ BR }

define_leakage \
	-pinlist {CLK D R Q} \
	-vector {1 0 0 1} \
	-when "CLK*!D*Q*!R" \
	{ BR }

define_leakage \
	-pinlist {CLK D R Q} \
	-vector {0 0 0 1} \
	-when "!CLK*!D*Q*!R" \
	{ BR }

define_arc \
	-type setup \
	-pinlist {CLK D R M Q} \
	-ic "0 $VDD 0 $VDD 0" \
	-vector {R F 0 X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ BR }

define_arc \
	-type hold \
	-pinlist {CLK D R M Q} \
	-ic "0 $VDD 0 $VDD 0" \
	-vector {R F 0 X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ BR }

define_arc \
	-type setup \
	-pinlist {CLK D R M Q} \
	-ic "0 0 0 0 0" \
	-vector {R R 0 X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ BR }

define_arc \
	-type hold \
	-pinlist {CLK D R M Q} \
	-ic "0 0 0 0 0" \
	-vector {R R 0 X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ BR }

define_arc \
	-type setup \
	-pinlist {CLK D R M Q} \
	-ic "0 $VDD $VDD 0 0" \
	-vector {R 1 F X X} \
	-related_pin CLK \
	-pin R \
	-probe {Q M} \
	{ BR }

define_arc \
	-type hold \
	-pinlist {CLK D R M Q} \
	-ic "0 $VDD $VDD 0 0" \
	-vector {R 1 F X X} \
	-related_pin CLK \
	-pin R \
	-probe {Q M} \
	{ BR }

define_arc \
	-type combinational \
	-pinlist {CLK D R Q} \
	-ic "$VDD $VDD 0 $VDD" \
	-vector {1 1 R F} \
	-related_pin R \
	-pin Q \
	{ SYNCR }

define_arc \
	-type edge \
	-pinlist {CLK D R Q} \
	-ic "0 $VDD 0 0" \
	-vector {R 1 0 R} \
	-related_pin CLK \
	-pin Q \
	{ SYNCR }

define_arc \
	-type edge \
	-pinlist {CLK D R Q} \
	-ic "0 0 0 $VDD" \
	-vector {R 0 0 F} \
	-related_pin CLK \
	-pin Q \
	{ SYNCR }

define_arc \
	-type hidden \
	-pinlist {CLK D R Q} \
	-ic "0 0 $VDD 0" \
	-vector {R 0 1 0} \
	-pin CLK \
	{ SYNCR }

define_arc \
	-type hidden \
	-pinlist {CLK D R Q} \
	-ic "0 0 $VDD 0" \
	-vector {0 R 1 0} \
	-pin D \
	{ SYNCR }

define_arc \
	-type hidden \
	-pinlist {CLK D R Q} \
	-ic "0 0 $VDD 0" \
	-vector {0 0 F 0} \
	-pin R \
	{ SYNCR }

define_arc \
	-type hidden \
	-pinlist {CLK D R Q} \
	-ic "0 $VDD $VDD 0" \
	-vector {0 F 1 0} \
	-pin D \
	{ SYNCR }

define_arc \
	-type hidden \
	-pinlist {CLK D R Q} \
	-ic "$VDD 0 $VDD 0" \
	-vector {F 0 1 0} \
	-pin CLK \
	{ SYNCR }

define_arc \
	-type hidden \
	-pinlist {CLK D R Q} \
	-ic "$VDD 0 0 0" \
	-vector {1 0 R 0} \
	-pin R \
	{ SYNCR }

define_leakage -when "!CLK*!D*!Q*R" { SYNCR }

define_leakage -when "!CLK*D*!Q*R" { SYNCR }

define_leakage -when "CLK*!D*!Q*R" { SYNCR }

define_leakage -when "CLK*D*!Q*R" { SYNCR }

define_leakage \
	-pinlist {CLK D R Q} \
	-vector {1 0 0 0} \
	-when "CLK*!D*!Q*!R" \
	{ SYNCR }

define_leakage \
	-pinlist {CLK D R Q} \
	-vector {1 1 0 1} \
	-when "CLK*D*Q*!R" \
	{ SYNCR }

define_leakage \
	-pinlist {CLK D R Q} \
	-vector {0 0 0 0} \
	-when "!CLK*!D*!Q*!R" \
	{ SYNCR }

define_leakage \
	-pinlist {CLK D R Q} \
	-vector {1 1 0 0} \
	-when "CLK*D*!Q*!R" \
	{ SYNCR }

define_leakage \
	-pinlist {CLK D R Q} \
	-vector {0 1 0 0} \
	-when "!CLK*D*!Q*!R" \
	{ SYNCR }

define_leakage \
	-pinlist {CLK D R Q} \
	-vector {1 0 0 1} \
	-when "CLK*!D*Q*!R" \
	{ SYNCR }

define_leakage \
	-pinlist {CLK D R Q} \
	-vector {0 1 0 1} \
	-when "!CLK*D*Q*!R" \
	{ SYNCR }

define_leakage \
	-pinlist {CLK D R Q} \
	-vector {0 0 0 1} \
	-when "!CLK*!D*Q*!R" \
	{ SYNCR }

define_arc \
	-type setup \
	-pinlist {CLK D R M Q} \
	-ic "0 $VDD 0 $VDD 0" \
	-vector {R F 0 X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ SYNCR }

define_arc \
	-type hold \
	-pinlist {CLK D R M Q} \
	-ic "0 $VDD 0 $VDD 0" \
	-vector {R F 0 X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ SYNCR }

define_arc \
	-type setup \
	-pinlist {CLK D R M Q} \
	-ic "0 0 0 0 0" \
	-vector {R R 0 X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ SYNCR }

define_arc \
	-type hold \
	-pinlist {CLK D R M Q} \
	-ic "0 0 0 0 0" \
	-vector {R R 0 X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ SYNCR }

define_arc \
	-type setup \
	-pinlist {CLK D R M Q} \
	-ic "0 $VDD $VDD 0 0" \
	-vector {R 1 F X X} \
	-related_pin CLK \
	-pin R \
	-probe {Q M} \
	{ SYNCR }

define_arc \
	-type hold \
	-pinlist {CLK D R M Q} \
	-ic "0 $VDD $VDD 0 0" \
	-vector {R 1 F X X} \
	-related_pin CLK \
	-pin R \
	-probe {Q M} \
	{ SYNCR }

define_arc \
	-type combinational \
	-pinlist {CLK D R G Q} \
	-ic "$VDD $VDD 0 0 $VDD" \
	-vector {1 1 R 0 F} \
	-related_pin R \
	-pin Q \
	{ SYNCRG }

define_arc \
	-type combinational \
	-pinlist {CLK D R G Q} \
	-ic "$VDD $VDD 0 0 $VDD" \
	-vector {1 1 0 R F} \
	-related_pin G \
	-pin Q \
	{ SYNCRG }

define_arc \
	-type edge \
	-pinlist {CLK D R G Q} \
	-ic "0 $VDD 0 0 0" \
	-vector {R 1 0 0 R} \
	-related_pin CLK \
	-pin Q \
	{ SYNCRG }

define_arc \
	-type edge \
	-pinlist {CLK D R G Q} \
	-ic "0 0 0 0 $VDD" \
	-vector {R 0 0 0 F} \
	-related_pin CLK \
	-pin Q \
	{ SYNCRG }

define_arc \
	-type hidden \
	-pinlist {CLK D R G Q} \
	-ic "0 0 $VDD 0 0" \
	-vector {R 0 1 0 0} \
	-pin CLK \
	{ SYNCRG }

define_arc \
	-type hidden \
	-pinlist {CLK D R G Q} \
	-ic "0 0 $VDD 0 0" \
	-vector {0 R 1 0 0} \
	-pin D \
	{ SYNCRG }

define_arc \
	-type hidden \
	-pinlist {CLK D R G Q} \
	-ic "0 0 $VDD 0 0" \
	-vector {0 0 F 0 0} \
	-pin R \
	{ SYNCRG }

define_arc \
	-type hidden \
	-pinlist {CLK D R G Q} \
	-ic "0 0 $VDD 0 0" \
	-vector {0 0 1 R 0} \
	-pin G \
	{ SYNCRG }

define_arc \
	-type hidden \
	-pinlist {CLK D R G Q} \
	-ic "0 0 0 $VDD 0" \
	-vector {0 0 R 1 0} \
	-pin R \
	{ SYNCRG }

define_arc \
	-type hidden \
	-pinlist {CLK D R G Q} \
	-ic "0 0 0 $VDD 0" \
	-vector {0 0 0 F 0} \
	-pin G \
	{ SYNCRG }

define_arc \
	-type hidden \
	-pinlist {CLK D R G Q} \
	-ic "0 $VDD $VDD 0 0" \
	-vector {0 F 1 0 0} \
	-pin D \
	{ SYNCRG }

define_arc \
	-type hidden \
	-pinlist {CLK D R G Q} \
	-ic "$VDD 0 $VDD 0 0" \
	-vector {F 0 1 0 0} \
	-pin CLK \
	{ SYNCRG }

define_leakage -when "!CLK*!D*!G*!Q*R" { SYNCRG }

define_leakage -when "!CLK*!D*G*!Q*!R" { SYNCRG }

define_leakage -when "!CLK*!D*G*!Q*R" { SYNCRG }

define_leakage -when "!CLK*D*!G*!Q*R" { SYNCRG }

define_leakage -when "!CLK*D*G*!Q*!R" { SYNCRG }

define_leakage -when "!CLK*D*G*!Q*R" { SYNCRG }

define_leakage -when "CLK*!D*!G*!Q*R" { SYNCRG }

define_leakage -when "CLK*!D*G*!Q*!R" { SYNCRG }

define_leakage -when "CLK*!D*G*!Q*R" { SYNCRG }

define_leakage -when "CLK*D*!G*!Q*R" { SYNCRG }

define_leakage -when "CLK*D*G*!Q*!R" { SYNCRG }

define_leakage -when "CLK*D*G*!Q*R" { SYNCRG }

define_leakage \
	-pinlist {CLK D R G Q} \
	-vector {1 1 0 0 0} \
	-when "CLK*D*!G*!Q*!R" \
	{ SYNCRG }

define_leakage \
	-pinlist {CLK D R G Q} \
	-vector {1 1 0 0 1} \
	-when "CLK*D*!G*Q*!R" \
	{ SYNCRG }

define_leakage \
	-pinlist {CLK D R G Q} \
	-vector {0 0 0 0 0} \
	-when "!CLK*!D*!G*!Q*!R" \
	{ SYNCRG }

define_leakage \
	-pinlist {CLK D R G Q} \
	-vector {1 0 0 0 0} \
	-when "CLK*!D*!G*!Q*!R" \
	{ SYNCRG }

define_leakage \
	-pinlist {CLK D R G Q} \
	-vector {0 1 0 0 0} \
	-when "!CLK*D*!G*!Q*!R" \
	{ SYNCRG }

define_leakage \
	-pinlist {CLK D R G Q} \
	-vector {0 1 0 0 1} \
	-when "!CLK*D*!G*Q*!R" \
	{ SYNCRG }

define_leakage \
	-pinlist {CLK D R G Q} \
	-vector {1 0 0 0 1} \
	-when "CLK*!D*!G*Q*!R" \
	{ SYNCRG }

define_leakage \
	-pinlist {CLK D R G Q} \
	-vector {0 0 0 0 1} \
	-when "!CLK*!D*!G*Q*!R" \
	{ SYNCRG }

define_arc \
	-type setup \
	-pinlist {CLK D R G M Q} \
	-ic "0 $VDD 0 0 $VDD 0" \
	-vector {R F 0 0 X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ SYNCRG }

define_arc \
	-type hold \
	-pinlist {CLK D R G M Q} \
	-ic "0 $VDD 0 0 $VDD 0" \
	-vector {R F 0 0 X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ SYNCRG }

define_arc \
	-type setup \
	-pinlist {CLK D R G M Q} \
	-ic "0 0 0 0 0 0" \
	-vector {R R 0 0 X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ SYNCRG }

define_arc \
	-type hold \
	-pinlist {CLK D R G M Q} \
	-ic "0 0 0 0 0 0" \
	-vector {R R 0 0 X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ SYNCRG }

define_arc \
	-type setup \
	-pinlist {CLK D R G M Q} \
	-ic "0 $VDD 0 $VDD 0 0" \
	-vector {R 1 0 F X X} \
	-related_pin CLK \
	-pin G \
	-probe {Q M} \
	{ SYNCRG }

define_arc \
	-type hold \
	-pinlist {CLK D R G M Q} \
	-ic "0 $VDD 0 $VDD 0 0" \
	-vector {R 1 0 F X X} \
	-related_pin CLK \
	-pin G \
	-probe {Q M} \
	{ SYNCRG }

define_arc \
	-type setup \
	-pinlist {CLK D R G M Q} \
	-ic "0 $VDD $VDD 0 0 0" \
	-vector {R 1 F 0 X X} \
	-related_pin CLK \
	-pin R \
	-probe {Q M} \
	{ SYNCRG }

define_arc \
	-type hold \
	-pinlist {CLK D R G M Q} \
	-ic "0 $VDD $VDD 0 0 0" \
	-vector {R 1 F 0 X X} \
	-related_pin CLK \
	-pin R \
	-probe {Q M} \
	{ SYNCRG }

define_arc \
	-type combinational \
	-pinlist {CLK D R G Q} \
	-ic "$VDD $VDD 0 $VDD $VDD" \
	-vector {1 1 R 1 F} \
	-related_pin R \
	-pin Q \
	{ GATEDR }

define_arc \
	-type edge \
	-pinlist {CLK D R G Q} \
	-ic "0 $VDD 0 $VDD 0" \
	-vector {R 1 0 1 R} \
	-related_pin CLK \
	-pin Q \
	{ GATEDR }

define_arc \
	-type combinational \
	-pinlist {CLK D R G Q} \
	-ic "$VDD $VDD $VDD 0 $VDD" \
	-vector {1 1 1 R F} \
	-related_pin G \
	-pin Q \
	{ GATEDR }

define_arc \
	-type edge \
	-pinlist {CLK D R G Q} \
	-ic "0 0 0 $VDD $VDD" \
	-vector {R 0 0 1 F} \
	-related_pin CLK \
	-pin Q \
	{ GATEDR }

define_arc \
	-type hidden \
	-pinlist {CLK D R G Q} \
	-ic "0 0 $VDD $VDD 0" \
	-vector {R 0 1 1 0} \
	-pin CLK \
	{ GATEDR }

define_arc \
	-type hidden \
	-pinlist {CLK D R G Q} \
	-ic "0 0 $VDD $VDD 0" \
	-vector {0 R 1 1 0} \
	-pin D \
	{ GATEDR }

define_arc \
	-type hidden \
	-pinlist {CLK D R G Q} \
	-ic "0 0 $VDD $VDD 0" \
	-vector {0 0 F 1 0} \
	-pin R \
	{ GATEDR }

define_arc \
	-type hidden \
	-pinlist {CLK D R G Q} \
	-ic "0 0 $VDD $VDD 0" \
	-vector {0 0 1 F 0} \
	-pin G \
	{ GATEDR }

define_arc \
	-type hidden \
	-pinlist {CLK D R G Q} \
	-ic "0 $VDD $VDD $VDD 0" \
	-vector {0 F 1 1 0} \
	-pin D \
	{ GATEDR }

define_arc \
	-type hidden \
	-pinlist {CLK D R G Q} \
	-ic "$VDD 0 $VDD $VDD 0" \
	-vector {F 0 1 1 0} \
	-pin CLK \
	{ GATEDR }

define_arc \
	-type hidden \
	-pinlist {CLK D R G Q} \
	-ic "0 0 0 $VDD 0" \
	-vector {0 0 R 1 0} \
	-pin R \
	{ GATEDR }

define_arc \
	-type hidden \
	-pinlist {CLK D R G Q} \
	-ic "$VDD $VDD $VDD 0 0" \
	-vector {1 1 1 R 0} \
	-pin G \
	{ GATEDR }

define_leakage -when "!CLK*!D*G*!Q*R" { GATEDR }

define_leakage -when "!CLK*D*G*!Q*R" { GATEDR }

define_leakage -when "CLK*!D*G*!Q*R" { GATEDR }

define_leakage -when "CLK*D*G*!Q*R" { GATEDR }

define_leakage \
	-pinlist {CLK D R G Q} \
	-vector {0 0 0 1 0} \
	-when "!CLK*!D*G*!Q*!R" \
	{ GATEDR }

define_leakage \
	-pinlist {CLK D R G Q} \
	-vector {1 1 1 0 0} \
	-when "CLK*D*!G*!Q*R" \
	{ GATEDR }

define_leakage \
	-pinlist {CLK D R G Q} \
	-vector {1 0 0 0 0} \
	-when "CLK*!D*!G*!Q*!R" \
	{ GATEDR }

define_leakage \
	-pinlist {CLK D R G Q} \
	-vector {1 0 1 0 0} \
	-when "CLK*!D*!G*!Q*R" \
	{ GATEDR }

define_leakage \
	-pinlist {CLK D R G Q} \
	-vector {0 0 1 0 0} \
	-when "!CLK*!D*!G*!Q*R" \
	{ GATEDR }

define_leakage \
	-pinlist {CLK D R G Q} \
	-vector {1 1 0 1 1} \
	-when "CLK*D*G*Q*!R" \
	{ GATEDR }

define_leakage \
	-pinlist {CLK D R G Q} \
	-vector {0 1 0 1 0} \
	-when "!CLK*D*G*!Q*!R" \
	{ GATEDR }

define_leakage \
	-pinlist {CLK D R G Q} \
	-vector {0 1 1 0 0} \
	-when "!CLK*D*!G*!Q*R" \
	{ GATEDR }

define_leakage \
	-pinlist {CLK D R G Q} \
	-vector {1 0 0 1 0} \
	-when "CLK*!D*G*!Q*!R" \
	{ GATEDR }

define_leakage \
	-pinlist {CLK D R G Q} \
	-vector {1 1 0 1 0} \
	-when "CLK*D*G*!Q*!R" \
	{ GATEDR }

define_leakage \
	-pinlist {CLK D R G Q} \
	-vector {1 1 0 0 1} \
	-when "CLK*D*!G*Q*!R" \
	{ GATEDR }

define_leakage \
	-pinlist {CLK D R G Q} \
	-vector {1 1 1 0 1} \
	-when "CLK*D*!G*Q*R" \
	{ GATEDR }

define_leakage \
	-pinlist {CLK D R G Q} \
	-vector {0 1 0 1 1} \
	-when "!CLK*D*G*Q*!R" \
	{ GATEDR }

define_leakage \
	-pinlist {CLK D R G Q} \
	-vector {1 0 0 0 1} \
	-when "CLK*!D*!G*Q*!R" \
	{ GATEDR }

define_leakage \
	-pinlist {CLK D R G Q} \
	-vector {0 1 1 0 1} \
	-when "!CLK*D*!G*Q*R" \
	{ GATEDR }

define_leakage \
	-pinlist {CLK D R G Q} \
	-vector {1 1 0 0 0} \
	-when "CLK*D*!G*!Q*!R" \
	{ GATEDR }

define_leakage \
	-pinlist {CLK D R G Q} \
	-vector {0 1 0 0 0} \
	-when "!CLK*D*!G*!Q*!R" \
	{ GATEDR }

define_leakage \
	-pinlist {CLK D R G Q} \
	-vector {0 0 0 0 0} \
	-when "!CLK*!D*!G*!Q*!R" \
	{ GATEDR }

define_leakage \
	-pinlist {CLK D R G Q} \
	-vector {1 0 0 1 1} \
	-when "CLK*!D*G*Q*!R" \
	{ GATEDR }

define_leakage \
	-pinlist {CLK D R G Q} \
	-vector {1 0 1 0 1} \
	-when "CLK*!D*!G*Q*R" \
	{ GATEDR }

define_leakage \
	-pinlist {CLK D R G Q} \
	-vector {0 1 0 0 1} \
	-when "!CLK*D*!G*Q*!R" \
	{ GATEDR }

define_leakage \
	-pinlist {CLK D R G Q} \
	-vector {0 0 0 1 1} \
	-when "!CLK*!D*G*Q*!R" \
	{ GATEDR }

define_leakage \
	-pinlist {CLK D R G Q} \
	-vector {0 0 0 0 1} \
	-when "!CLK*!D*!G*Q*!R" \
	{ GATEDR }

define_leakage \
	-pinlist {CLK D R G Q} \
	-vector {0 0 1 0 1} \
	-when "!CLK*!D*!G*Q*R" \
	{ GATEDR }

define_arc \
	-type non_seq_setup \
	-pinlist {CLK D R G Q} \
	-ic "0 $VDD 0 $VDD $VDD" \
	-vector {0 1 R F X} \
	-related_pin R \
	-pin G \
	-probe {Q} \
	{ GATEDR }

define_arc \
	-type non_seq_hold \
	-pinlist {CLK D R G Q} \
	-ic "0 $VDD 0 $VDD $VDD" \
	-vector {0 1 R F X} \
	-related_pin R \
	-pin G \
	-probe {Q} \
	{ GATEDR }

define_arc \
	-type non_seq_setup \
	-pinlist {CLK D R G M Q} \
	-ic "$VDD $VDD 0 $VDD $VDD $VDD" \
	-vector {1 1 R F X X} \
	-related_pin R \
	-pin G \
	-probe {Q M} \
	{ GATEDR }

define_arc \
	-type non_seq_hold \
	-pinlist {CLK D R G M Q} \
	-ic "$VDD $VDD 0 $VDD $VDD $VDD" \
	-vector {1 1 R F X X} \
	-related_pin R \
	-pin G \
	-probe {Q M} \
	{ GATEDR }

define_arc \
	-type non_seq_setup \
	-pinlist {CLK D R G Q} \
	-ic "0 $VDD $VDD 0 $VDD" \
	-vector {0 1 F R X} \
	-related_pin R \
	-pin G \
	-probe {Q} \
	{ GATEDR }

define_arc \
	-type non_seq_hold \
	-pinlist {CLK D R G Q} \
	-ic "0 $VDD $VDD 0 $VDD" \
	-vector {0 1 F R X} \
	-related_pin R \
	-pin G \
	-probe {Q} \
	{ GATEDR }

define_arc \
	-type non_seq_setup \
	-pinlist {CLK D R G M Q} \
	-ic "$VDD $VDD $VDD 0 $VDD $VDD" \
	-vector {1 1 F R X X} \
	-related_pin R \
	-pin G \
	-probe {Q M} \
	{ GATEDR }

define_arc \
	-type non_seq_hold \
	-pinlist {CLK D R G M Q} \
	-ic "$VDD $VDD $VDD 0 $VDD $VDD" \
	-vector {1 1 F R X X} \
	-related_pin R \
	-pin G \
	-probe {Q M} \
	{ GATEDR }

define_arc \
	-type setup \
	-pinlist {CLK D R G M Q} \
	-ic "0 $VDD 0 $VDD $VDD 0" \
	-vector {R F 0 1 X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ GATEDR }

define_arc \
	-type hold \
	-pinlist {CLK D R G M Q} \
	-ic "0 $VDD 0 $VDD $VDD 0" \
	-vector {R F 0 1 X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ GATEDR }

define_arc \
	-type setup \
	-pinlist {CLK D R G M Q} \
	-ic "0 0 0 $VDD 0 0" \
	-vector {R R 0 1 X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ GATEDR }

define_arc \
	-type hold \
	-pinlist {CLK D R G M Q} \
	-ic "0 0 0 $VDD 0 0" \
	-vector {R R 0 1 X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ GATEDR }

define_arc \
	-type setup \
	-pinlist {CLK D R G M Q} \
	-ic "0 $VDD $VDD $VDD 0 0" \
	-vector {R 1 1 F X X} \
	-related_pin CLK \
	-pin G \
	-probe {Q M} \
	{ GATEDR }

define_arc \
	-type hold \
	-pinlist {CLK D R G M Q} \
	-ic "0 $VDD $VDD $VDD 0 0" \
	-vector {R 1 1 F X X} \
	-related_pin CLK \
	-pin G \
	-probe {Q M} \
	{ GATEDR }

define_arc \
	-type setup \
	-pinlist {CLK D R G M Q} \
	-ic "0 $VDD $VDD $VDD 0 0" \
	-vector {R 1 F 1 X X} \
	-related_pin CLK \
	-pin R \
	-probe {Q M} \
	{ GATEDR }

define_arc \
	-type hold \
	-pinlist {CLK D R G M Q} \
	-ic "0 $VDD $VDD $VDD 0 0" \
	-vector {R 1 F 1 X X} \
	-related_pin CLK \
	-pin R \
	-probe {Q M} \
	{ GATEDR }

define_arc \
	-type async \
	-pinlist {CLK D R G Q} \
	-ic "$VDD $VDD $VDD 0 $VDD" \
	-vector {1 1 1 R F} \
	-related_pin G \
	-pin Q \
	{ AGATEDR }

define_arc \
	-type async \
	-pinlist {CLK D R G Q} \
	-ic "$VDD $VDD 0 $VDD $VDD" \
	-vector {1 1 R 1 F} \
	-related_pin R \
	-pin Q \
	{ AGATEDR }

define_arc \
	-type edge \
	-pinlist {CLK D R G Q} \
	-ic "0 $VDD $VDD 0 0" \
	-vector {R 1 1 0 R} \
	-related_pin CLK \
	-pin Q \
	{ AGATEDR }

define_arc \
	-type edge \
	-pinlist {CLK D R G Q} \
	-ic "0 0 $VDD 0 $VDD" \
	-vector {R 0 1 0 F} \
	-related_pin CLK \
	-pin Q \
	{ AGATEDR }

define_arc \
	-type hidden \
	-pinlist {CLK D R G Q} \
	-ic "0 0 $VDD $VDD 0" \
	-vector {R 0 1 1 0} \
	-pin CLK \
	{ AGATEDR }

define_arc \
	-type hidden \
	-pinlist {CLK D R G Q} \
	-ic "0 0 $VDD $VDD 0" \
	-vector {0 R 1 1 0} \
	-pin D \
	{ AGATEDR }

define_arc \
	-type hidden \
	-pinlist {CLK D R G Q} \
	-ic "0 0 $VDD $VDD 0" \
	-vector {0 0 F 1 0} \
	-pin R \
	{ AGATEDR }

define_arc \
	-type hidden \
	-pinlist {CLK D R G Q} \
	-ic "0 0 $VDD $VDD 0" \
	-vector {0 0 1 F 0} \
	-pin G \
	{ AGATEDR }

define_arc \
	-type hidden \
	-pinlist {CLK D R G Q} \
	-ic "0 $VDD $VDD $VDD 0" \
	-vector {0 F 1 1 0} \
	-pin D \
	{ AGATEDR }

define_arc \
	-type hidden \
	-pinlist {CLK D R G Q} \
	-ic "$VDD 0 $VDD $VDD 0" \
	-vector {F 0 1 1 0} \
	-pin CLK \
	{ AGATEDR }

define_arc \
	-type hidden \
	-pinlist {CLK D R G Q} \
	-ic "$VDD 0 $VDD 0 0" \
	-vector {1 0 1 R 0} \
	-pin G \
	{ AGATEDR }

define_arc \
	-type hidden \
	-pinlist {CLK D R G Q} \
	-ic "$VDD $VDD 0 0 $VDD" \
	-vector {1 1 R 0 1} \
	-pin R \
	{ AGATEDR }

define_leakage -when "!CLK*!D*G*!Q*R" { AGATEDR }

define_leakage -when "!CLK*D*G*!Q*R" { AGATEDR }

define_leakage -when "CLK*!D*G*!Q*R" { AGATEDR }

define_leakage -when "CLK*D*G*!Q*R" { AGATEDR }

define_leakage \
	-pinlist {CLK D R G Q} \
	-vector {1 0 1 0 0} \
	-when "CLK*!D*!G*!Q*R" \
	{ AGATEDR }

define_leakage \
	-pinlist {CLK D R G Q} \
	-vector {1 1 0 0 1} \
	-when "CLK*D*!G*Q*!R" \
	{ AGATEDR }

define_leakage \
	-pinlist {CLK D R G Q} \
	-vector {1 1 1 0 1} \
	-when "CLK*D*!G*Q*R" \
	{ AGATEDR }

define_leakage \
	-pinlist {CLK D R G Q} \
	-vector {1 0 0 0 0} \
	-when "CLK*!D*!G*!Q*!R" \
	{ AGATEDR }

define_leakage \
	-pinlist {CLK D R G Q} \
	-vector {1 1 0 1 1} \
	-when "CLK*D*G*Q*!R" \
	{ AGATEDR }

define_leakage \
	-pinlist {CLK D R G Q} \
	-vector {0 1 1 0 0} \
	-when "!CLK*D*!G*!Q*R" \
	{ AGATEDR }

define_leakage \
	-pinlist {CLK D R G Q} \
	-vector {1 0 0 1 0} \
	-when "CLK*!D*G*!Q*!R" \
	{ AGATEDR }

define_leakage \
	-pinlist {CLK D R G Q} \
	-vector {1 1 0 1 0} \
	-when "CLK*D*G*!Q*!R" \
	{ AGATEDR }

define_leakage \
	-pinlist {CLK D R G Q} \
	-vector {0 0 0 1 0} \
	-when "!CLK*!D*G*!Q*!R" \
	{ AGATEDR }

define_leakage \
	-pinlist {CLK D R G Q} \
	-vector {0 0 1 0 0} \
	-when "!CLK*!D*!G*!Q*R" \
	{ AGATEDR }

define_leakage \
	-pinlist {CLK D R G Q} \
	-vector {1 1 1 0 0} \
	-when "CLK*D*!G*!Q*R" \
	{ AGATEDR }

define_leakage \
	-pinlist {CLK D R G Q} \
	-vector {0 1 0 1 0} \
	-when "!CLK*D*G*!Q*!R" \
	{ AGATEDR }

define_leakage \
	-pinlist {CLK D R G Q} \
	-vector {0 1 1 0 1} \
	-when "!CLK*D*!G*Q*R" \
	{ AGATEDR }

define_leakage \
	-pinlist {CLK D R G Q} \
	-vector {0 1 0 0 0} \
	-when "!CLK*D*!G*!Q*!R" \
	{ AGATEDR }

define_leakage \
	-pinlist {CLK D R G Q} \
	-vector {0 1 0 0 1} \
	-when "!CLK*D*!G*Q*!R" \
	{ AGATEDR }

define_leakage \
	-pinlist {CLK D R G Q} \
	-vector {1 1 0 0 0} \
	-when "CLK*D*!G*!Q*!R" \
	{ AGATEDR }

define_leakage \
	-pinlist {CLK D R G Q} \
	-vector {1 0 0 0 1} \
	-when "CLK*!D*!G*Q*!R" \
	{ AGATEDR }

define_leakage \
	-pinlist {CLK D R G Q} \
	-vector {0 0 0 0 0} \
	-when "!CLK*!D*!G*!Q*!R" \
	{ AGATEDR }

define_leakage \
	-pinlist {CLK D R G Q} \
	-vector {0 1 0 1 1} \
	-when "!CLK*D*G*Q*!R" \
	{ AGATEDR }

define_leakage \
	-pinlist {CLK D R G Q} \
	-vector {1 0 0 1 1} \
	-when "CLK*!D*G*Q*!R" \
	{ AGATEDR }

define_leakage \
	-pinlist {CLK D R G Q} \
	-vector {1 0 1 0 1} \
	-when "CLK*!D*!G*Q*R" \
	{ AGATEDR }

define_leakage \
	-pinlist {CLK D R G Q} \
	-vector {0 0 1 0 1} \
	-when "!CLK*!D*!G*Q*R" \
	{ AGATEDR }

define_leakage \
	-pinlist {CLK D R G Q} \
	-vector {0 0 0 0 1} \
	-when "!CLK*!D*!G*Q*!R" \
	{ AGATEDR }

define_leakage \
	-pinlist {CLK D R G Q} \
	-vector {0 0 0 1 1} \
	-when "!CLK*!D*G*Q*!R" \
	{ AGATEDR }

define_arc \
	-type non_seq_setup \
	-pinlist {CLK D R G Q} \
	-ic "0 $VDD 0 $VDD $VDD" \
	-vector {0 1 R F X} \
	-related_pin R \
	-pin G \
	-probe {Q} \
	{ AGATEDR }

define_arc \
	-type non_seq_hold \
	-pinlist {CLK D R G Q} \
	-ic "0 $VDD 0 $VDD $VDD" \
	-vector {0 1 R F X} \
	-related_pin R \
	-pin G \
	-probe {Q} \
	{ AGATEDR }

define_arc \
	-type non_seq_setup \
	-pinlist {CLK D R G M Q} \
	-ic "$VDD $VDD 0 $VDD $VDD $VDD" \
	-vector {1 1 R F X X} \
	-related_pin R \
	-pin G \
	-probe {Q M} \
	{ AGATEDR }

define_arc \
	-type non_seq_hold \
	-pinlist {CLK D R G M Q} \
	-ic "$VDD $VDD 0 $VDD $VDD $VDD" \
	-vector {1 1 R F X X} \
	-related_pin R \
	-pin G \
	-probe {Q M} \
	{ AGATEDR }

define_arc \
	-type non_seq_setup \
	-pinlist {CLK D R G Q} \
	-ic "0 $VDD $VDD 0 $VDD" \
	-vector {0 1 F R X} \
	-related_pin R \
	-pin G \
	-probe {Q} \
	{ AGATEDR }

define_arc \
	-type non_seq_hold \
	-pinlist {CLK D R G Q} \
	-ic "0 $VDD $VDD 0 $VDD" \
	-vector {0 1 F R X} \
	-related_pin R \
	-pin G \
	-probe {Q} \
	{ AGATEDR }

define_arc \
	-type non_seq_setup \
	-pinlist {CLK D R G M Q} \
	-ic "$VDD $VDD $VDD 0 $VDD $VDD" \
	-vector {1 1 F R X X} \
	-related_pin R \
	-pin G \
	-probe {Q M} \
	{ AGATEDR }

define_arc \
	-type non_seq_hold \
	-pinlist {CLK D R G M Q} \
	-ic "$VDD $VDD $VDD 0 $VDD $VDD" \
	-vector {1 1 F R X X} \
	-related_pin R \
	-pin G \
	-probe {Q M} \
	{ AGATEDR }

define_arc \
	-type setup \
	-pinlist {CLK D R G M Q} \
	-ic "0 $VDD $VDD 0 $VDD 0" \
	-vector {R F 1 0 X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ AGATEDR }

define_arc \
	-type hold \
	-pinlist {CLK D R G M Q} \
	-ic "0 $VDD $VDD 0 $VDD 0" \
	-vector {R F 1 0 X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ AGATEDR }

define_arc \
	-type setup \
	-pinlist {CLK D R G M Q} \
	-ic "0 0 0 $VDD 0 0" \
	-vector {R R 0 1 X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ AGATEDR }

define_arc \
	-type hold \
	-pinlist {CLK D R G M Q} \
	-ic "0 0 0 $VDD 0 0" \
	-vector {R R 0 1 X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ AGATEDR }

define_arc \
	-type setup \
	-pinlist {CLK D R G M Q} \
	-ic "0 $VDD $VDD $VDD 0 0" \
	-vector {R 1 1 F X X} \
	-related_pin CLK \
	-pin G \
	-probe {Q M} \
	{ AGATEDR }

define_arc \
	-type hold \
	-pinlist {CLK D R G M Q} \
	-ic "0 $VDD $VDD $VDD 0 0" \
	-vector {R 1 1 F X X} \
	-related_pin CLK \
	-pin G \
	-probe {Q M} \
	{ AGATEDR }

define_arc \
	-type setup \
	-pinlist {CLK D R G M Q} \
	-ic "0 $VDD $VDD $VDD 0 0" \
	-vector {R 1 F 1 X X} \
	-related_pin CLK \
	-pin R \
	-probe {Q M} \
	{ AGATEDR }

define_arc \
	-type hold \
	-pinlist {CLK D R G M Q} \
	-ic "0 $VDD $VDD $VDD 0 0" \
	-vector {R 1 F 1 X X} \
	-related_pin CLK \
	-pin R \
	-probe {Q M} \
	{ AGATEDR }

define_arc \
	-type combinational \
	-pinlist {CLK D R Q} \
	-ic "$VDD $VDD 0 $VDD" \
	-vector {1 1 R F} \
	-related_pin R \
	-pin Q \
	{ RDFF }

define_arc \
	-type edge \
	-pinlist {CLK D R Q} \
	-ic "0 $VDD 0 0" \
	-vector {R 1 0 R} \
	-related_pin CLK \
	-pin Q \
	{ RDFF }

define_arc \
	-type edge \
	-pinlist {CLK D R Q} \
	-ic "0 0 0 $VDD" \
	-vector {R 0 0 F} \
	-related_pin CLK \
	-pin Q \
	{ RDFF }

define_arc \
	-type hidden \
	-pinlist {CLK D R Q} \
	-ic "0 0 $VDD 0" \
	-vector {R 0 1 0} \
	-pin CLK \
	{ RDFF }

define_arc \
	-type hidden \
	-pinlist {CLK D R Q} \
	-ic "0 0 $VDD 0" \
	-vector {0 R 1 0} \
	-pin D \
	{ RDFF }

define_arc \
	-type hidden \
	-pinlist {CLK D R Q} \
	-ic "0 0 $VDD 0" \
	-vector {0 0 F 0} \
	-pin R \
	{ RDFF }

define_arc \
	-type hidden \
	-pinlist {CLK D R Q} \
	-ic "0 $VDD $VDD 0" \
	-vector {0 F 1 0} \
	-pin D \
	{ RDFF }

define_arc \
	-type hidden \
	-pinlist {CLK D R Q} \
	-ic "$VDD 0 $VDD 0" \
	-vector {F 0 1 0} \
	-pin CLK \
	{ RDFF }

define_arc \
	-type hidden \
	-pinlist {CLK D R Q} \
	-ic "$VDD $VDD 0 0" \
	-vector {1 1 R 0} \
	-pin R \
	{ RDFF }

define_leakage -when "!CLK*!D*!Q*R" { RDFF }

define_leakage -when "!CLK*D*!Q*R" { RDFF }

define_leakage -when "CLK*!D*!Q*R" { RDFF }

define_leakage -when "CLK*D*!Q*R" { RDFF }

define_leakage \
	-pinlist {CLK D R Q} \
	-vector {1 1 0 0} \
	-when "CLK*D*!Q*!R" \
	{ RDFF }

define_leakage \
	-pinlist {CLK D R Q} \
	-vector {0 0 0 0} \
	-when "!CLK*!D*!Q*!R" \
	{ RDFF }

define_leakage \
	-pinlist {CLK D R Q} \
	-vector {1 1 0 1} \
	-when "CLK*D*Q*!R" \
	{ RDFF }

define_leakage \
	-pinlist {CLK D R Q} \
	-vector {1 0 0 0} \
	-when "CLK*!D*!Q*!R" \
	{ RDFF }

define_leakage \
	-pinlist {CLK D R Q} \
	-vector {0 1 0 0} \
	-when "!CLK*D*!Q*!R" \
	{ RDFF }

define_leakage \
	-pinlist {CLK D R Q} \
	-vector {0 1 0 1} \
	-when "!CLK*D*Q*!R" \
	{ RDFF }

define_leakage \
	-pinlist {CLK D R Q} \
	-vector {1 0 0 1} \
	-when "CLK*!D*Q*!R" \
	{ RDFF }

define_leakage \
	-pinlist {CLK D R Q} \
	-vector {0 0 0 1} \
	-when "!CLK*!D*Q*!R" \
	{ RDFF }

define_arc \
	-type non_seq_setup \
	-pinlist {CLK D R M Q} \
	-ic "0 $VDD $VDD 0 0" \
	-vector {R 1 F X X} \
	-related_pin CLK \
	-pin R \
	-probe {Q M} \
	{ RDFF }

define_arc \
	-type non_seq_hold \
	-pinlist {CLK D R M Q} \
	-ic "0 $VDD $VDD 0 0" \
	-vector {R 1 F X X} \
	-related_pin CLK \
	-pin R \
	-probe {Q M} \
	{ RDFF }

define_arc \
	-type setup \
	-pinlist {CLK D R M Q} \
	-ic "0 $VDD 0 $VDD 0" \
	-vector {R F 0 X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ RDFF }

define_arc \
	-type hold \
	-pinlist {CLK D R M Q} \
	-ic "0 $VDD 0 $VDD 0" \
	-vector {R F 0 X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ RDFF }

define_arc \
	-type setup \
	-pinlist {CLK D R M Q} \
	-ic "0 0 0 0 0" \
	-vector {R R 0 X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ RDFF }

define_arc \
	-type hold \
	-pinlist {CLK D R M Q} \
	-ic "0 0 0 0 0" \
	-vector {R R 0 X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ RDFF }

define_arc \
	-type async \
	-pinlist {CLK D B R Q} \
	-ic "0 0 $VDD 0 $VDD" \
	-vector {0 0 1 R F} \
	-related_pin R \
	-pin Q \
	{ COEX }

define_arc \
	-type async \
	-pinlist {CLK D B R Q} \
	-ic "0 0 $VDD $VDD 0" \
	-vector {0 0 1 F R} \
	-related_pin R \
	-pin Q \
	{ COEX }

define_arc \
	-type edge \
	-pinlist {CLK D B R Q} \
	-ic "0 $VDD 0 0 0" \
	-vector {R 1 0 0 R} \
	-related_pin CLK \
	-pin Q \
	{ COEX }

define_arc \
	-type combinational \
	-pinlist {CLK D B R Q} \
	-ic "0 $VDD 0 0 0" \
	-vector {0 1 R 0 R} \
	-related_pin B \
	-pin Q \
	{ COEX }

define_arc \
	-type edge \
	-pinlist {CLK D B R Q} \
	-ic "0 0 0 0 $VDD" \
	-vector {R 0 0 0 F} \
	-related_pin CLK \
	-pin Q \
	{ COEX }

define_arc \
	-type hidden \
	-pinlist {CLK D B R Q} \
	-ic "0 0 0 $VDD 0" \
	-vector {R 0 0 1 0} \
	-pin CLK \
	{ COEX }

define_arc \
	-type hidden \
	-pinlist {CLK D B R Q} \
	-ic "0 0 0 $VDD 0" \
	-vector {0 R 0 1 0} \
	-pin D \
	{ COEX }

define_arc \
	-type hidden \
	-pinlist {CLK D B R Q} \
	-ic "0 0 0 $VDD 0" \
	-vector {0 0 R 1 0} \
	-pin B \
	{ COEX }

define_arc \
	-type hidden \
	-pinlist {CLK D B R Q} \
	-ic "0 0 0 $VDD 0" \
	-vector {0 0 0 F 0} \
	-pin R \
	{ COEX }

define_arc \
	-type hidden \
	-pinlist {CLK D B R Q} \
	-ic "0 $VDD 0 $VDD 0" \
	-vector {0 F 0 1 0} \
	-pin D \
	{ COEX }

define_arc \
	-type hidden \
	-pinlist {CLK D B R Q} \
	-ic "$VDD 0 0 $VDD 0" \
	-vector {F 0 0 1 0} \
	-pin CLK \
	{ COEX }

define_arc \
	-type hidden \
	-pinlist {CLK D B R Q} \
	-ic "0 0 $VDD 0 $VDD" \
	-vector {0 0 F 0 1} \
	-pin B \
	{ COEX }

define_arc \
	-type hidden \
	-pinlist {CLK D B R Q} \
	-ic "0 $VDD 0 0 0" \
	-vector {0 1 0 R 0} \
	-pin R \
	{ COEX }

define_leakage -when "!B*!CLK*!D*!Q*R" { COEX }

define_leakage -when "!B*!CLK*D*!Q*R" { COEX }

define_leakage -when "!B*CLK*!D*!Q*R" { COEX }

define_leakage -when "!B*CLK*D*!Q*R" { COEX }

define_leakage -when "B*!CLK*!D*Q*!R" { COEX }

define_leakage -when "B*!CLK*!D*!Q*R" { COEX }

define_leakage -when "B*!CLK*D*Q*!R" { COEX }

define_leakage -when "B*!CLK*D*!Q*R" { COEX }

define_leakage -when "B*CLK*!D*Q*!R" { COEX }

define_leakage -when "B*CLK*!D*!Q*R" { COEX }

define_leakage -when "B*CLK*D*Q*!R" { COEX }

define_leakage -when "B*CLK*D*!Q*R" { COEX }

define_leakage \
	-pinlist {CLK D B R Q} \
	-vector {0 1 0 0 0} \
	-when "!B*!CLK*D*!Q*!R" \
	{ COEX }

define_leakage \
	-pinlist {CLK D B R Q} \
	-vector {0 0 0 0 1} \
	-when "!B*!CLK*!D*Q*!R" \
	{ COEX }

define_leakage \
	-pinlist {CLK D B R Q} \
	-vector {0 0 0 0 0} \
	-when "!B*!CLK*!D*!Q*!R" \
	{ COEX }

define_leakage \
	-pinlist {CLK D B R Q} \
	-vector {0 1 0 0 1} \
	-when "!B*!CLK*D*Q*!R" \
	{ COEX }

define_leakage \
	-pinlist {CLK D B R Q} \
	-vector {1 0 0 0 1} \
	-when "!B*CLK*!D*Q*!R" \
	{ COEX }

define_leakage \
	-pinlist {CLK D B R Q} \
	-vector {1 1 0 0 1} \
	-when "!B*CLK*D*Q*!R" \
	{ COEX }

define_leakage \
	-pinlist {CLK D B R Q} \
	-vector {1 1 0 0 0} \
	-when "!B*CLK*D*!Q*!R" \
	{ COEX }

define_leakage \
	-pinlist {CLK D B R Q} \
	-vector {1 0 0 0 0} \
	-when "!B*CLK*!D*!Q*!R" \
	{ COEX }

define_arc \
	-type non_seq_setup \
	-pinlist {CLK D B R Q} \
	-ic "0 0 $VDD $VDD 0" \
	-vector {0 0 F F X} \
	-related_pin B \
	-pin R \
	-probe {Q} \
	{ COEX }

define_arc \
	-type non_seq_hold \
	-pinlist {CLK D B R Q} \
	-ic "0 0 $VDD $VDD 0" \
	-vector {0 0 F F X} \
	-related_pin B \
	-pin R \
	-probe {Q} \
	{ COEX }

define_arc \
	-type non_seq_setup \
	-pinlist {CLK D B R M Q} \
	-ic "$VDD 0 $VDD $VDD 0 0" \
	-vector {1 0 F F X X} \
	-related_pin B \
	-pin R \
	-probe {Q M} \
	{ COEX }

define_arc \
	-type non_seq_hold \
	-pinlist {CLK D B R M Q} \
	-ic "$VDD 0 $VDD $VDD 0 0" \
	-vector {1 0 F F X X} \
	-related_pin B \
	-pin R \
	-probe {Q M} \
	{ COEX }

define_arc \
	-type setup \
	-pinlist {CLK D B R M Q} \
	-ic "0 0 $VDD 0 $VDD $VDD" \
	-vector {R 0 F 0 X X} \
	-related_pin CLK \
	-pin B \
	-probe {Q M} \
	{ COEX }

define_arc \
	-type hold \
	-pinlist {CLK D B R M Q} \
	-ic "0 0 $VDD 0 $VDD $VDD" \
	-vector {R 0 F 0 X X} \
	-related_pin CLK \
	-pin B \
	-probe {Q M} \
	{ COEX }

define_arc \
	-type setup \
	-pinlist {CLK D B R M Q} \
	-ic "0 $VDD 0 0 $VDD 0" \
	-vector {R F 0 0 X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ COEX }

define_arc \
	-type hold \
	-pinlist {CLK D B R M Q} \
	-ic "0 $VDD 0 0 $VDD 0" \
	-vector {R F 0 0 X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ COEX }

define_arc \
	-type setup \
	-pinlist {CLK D B R M Q} \
	-ic "0 0 0 0 0 $VDD" \
	-vector {R R 0 0 X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ COEX }

define_arc \
	-type hold \
	-pinlist {CLK D B R M Q} \
	-ic "0 0 0 0 0 $VDD" \
	-vector {R R 0 0 X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ COEX }

define_arc \
	-type setup \
	-pinlist {CLK D B R M Q} \
	-ic "0 $VDD 0 $VDD 0 0" \
	-vector {R 1 0 F X X} \
	-related_pin CLK \
	-pin R \
	-probe {Q M} \
	{ COEX }

define_arc \
	-type hold \
	-pinlist {CLK D B R M Q} \
	-ic "0 $VDD 0 $VDD 0 0" \
	-vector {R 1 0 F X X} \
	-related_pin CLK \
	-pin R \
	-probe {Q M} \
	{ COEX }

define_arc \
	-type async \
	-pinlist {CLK D PRE CLR Q} \
	-ic "0 0 $VDD 0 $VDD" \
	-vector {0 0 1 R F} \
	-related_pin CLR \
	-pin Q \
	{ CAFF }

define_arc \
	-type async \
	-pinlist {CLK D PRE CLR Q} \
	-ic "0 0 $VDD $VDD 0" \
	-vector {0 0 1 F R} \
	-related_pin CLR \
	-pin Q \
	{ CAFF }

define_arc \
	-type async \
	-pinlist {CLK D PRE CLR Q} \
	-ic "0 0 0 0 0" \
	-vector {0 0 R 0 R} \
	-related_pin PRE \
	-pin Q \
	{ CAFF }

define_arc \
	-type edge \
	-pinlist {CLK D PRE CLR Q} \
	-ic "0 $VDD 0 0 0" \
	-vector {R 1 0 0 R} \
	-related_pin CLK \
	-pin Q \
	{ CAFF }

define_arc \
	-type edge \
	-pinlist {CLK D PRE CLR Q} \
	-ic "0 0 0 0 $VDD" \
	-vector {R 0 0 0 F} \
	-related_pin CLK \
	-pin Q \
	{ CAFF }

define_arc \
	-type hidden \
	-pinlist {CLK D PRE CLR Q} \
	-ic "0 0 $VDD 0 $VDD" \
	-vector {R 0 1 0 1} \
	-pin CLK \
	{ CAFF }

define_arc \
	-type hidden \
	-pinlist {CLK D PRE CLR Q} \
	-ic "0 0 $VDD 0 $VDD" \
	-vector {0 R 1 0 1} \
	-pin D \
	{ CAFF }

define_arc \
	-type hidden \
	-pinlist {CLK D PRE CLR Q} \
	-ic "0 0 $VDD 0 $VDD" \
	-vector {0 0 F 0 1} \
	-pin PRE \
	{ CAFF }

define_arc \
	-type hidden \
	-pinlist {CLK D PRE CLR Q} \
	-ic "0 $VDD $VDD 0 $VDD" \
	-vector {0 F 1 0 1} \
	-pin D \
	{ CAFF }

define_arc \
	-type hidden \
	-pinlist {CLK D PRE CLR Q} \
	-ic "0 0 0 $VDD 0" \
	-vector {0 0 R 1 0} \
	-pin PRE \
	{ CAFF }

define_arc \
	-type hidden \
	-pinlist {CLK D PRE CLR Q} \
	-ic "0 0 0 $VDD 0" \
	-vector {0 0 0 F 0} \
	-pin CLR \
	{ CAFF }

define_arc \
	-type hidden \
	-pinlist {CLK D PRE CLR Q} \
	-ic "$VDD 0 $VDD 0 $VDD" \
	-vector {F 0 1 0 1} \
	-pin CLK \
	{ CAFF }

define_arc \
	-type hidden \
	-pinlist {CLK D PRE CLR Q} \
	-ic "0 0 0 0 0" \
	-vector {0 0 0 R 0} \
	-pin CLR \
	{ CAFF }

define_leakage -when "!CLK*!CLR*!D*PRE*Q" { CAFF }

define_leakage -when "!CLK*!CLR*D*PRE*Q" { CAFF }

define_leakage -when "!CLK*CLR*!D*!PRE*!Q" { CAFF }

define_leakage -when "!CLK*CLR*!D*PRE*!Q" { CAFF }

define_leakage -when "!CLK*CLR*D*!PRE*!Q" { CAFF }

define_leakage -when "!CLK*CLR*D*PRE*!Q" { CAFF }

define_leakage -when "CLK*!CLR*!D*PRE*Q" { CAFF }

define_leakage -when "CLK*!CLR*D*PRE*Q" { CAFF }

define_leakage -when "CLK*CLR*!D*!PRE*!Q" { CAFF }

define_leakage -when "CLK*CLR*!D*PRE*!Q" { CAFF }

define_leakage -when "CLK*CLR*D*!PRE*!Q" { CAFF }

define_leakage -when "CLK*CLR*D*PRE*!Q" { CAFF }

define_leakage \
	-pinlist {CLK D PRE CLR Q} \
	-vector {1 0 0 0 1} \
	-when "CLK*!CLR*!D*!PRE*Q" \
	{ CAFF }

define_leakage \
	-pinlist {CLK D PRE CLR Q} \
	-vector {1 1 0 0 1} \
	-when "CLK*!CLR*D*!PRE*Q" \
	{ CAFF }

define_leakage \
	-pinlist {CLK D PRE CLR Q} \
	-vector {0 0 0 0 0} \
	-when "!CLK*!CLR*!D*!PRE*!Q" \
	{ CAFF }

define_leakage \
	-pinlist {CLK D PRE CLR Q} \
	-vector {0 1 0 0 1} \
	-when "!CLK*!CLR*D*!PRE*Q" \
	{ CAFF }

define_leakage \
	-pinlist {CLK D PRE CLR Q} \
	-vector {0 1 0 0 0} \
	-when "!CLK*!CLR*D*!PRE*!Q" \
	{ CAFF }

define_leakage \
	-pinlist {CLK D PRE CLR Q} \
	-vector {1 0 0 0 0} \
	-when "CLK*!CLR*!D*!PRE*!Q" \
	{ CAFF }

define_leakage \
	-pinlist {CLK D PRE CLR Q} \
	-vector {1 1 0 0 0} \
	-when "CLK*!CLR*D*!PRE*!Q" \
	{ CAFF }

define_leakage \
	-pinlist {CLK D PRE CLR Q} \
	-vector {0 0 0 0 1} \
	-when "!CLK*!CLR*!D*!PRE*Q" \
	{ CAFF }

define_arc \
	-type non_seq_setup \
	-pinlist {CLK D PRE CLR Q} \
	-ic "0 0 $VDD $VDD 0" \
	-vector {0 0 F F X} \
	-related_pin PRE \
	-pin CLR \
	-probe {Q} \
	{ CAFF }

define_arc \
	-type non_seq_hold \
	-pinlist {CLK D PRE CLR Q} \
	-ic "0 0 $VDD $VDD 0" \
	-vector {0 0 F F X} \
	-related_pin PRE \
	-pin CLR \
	-probe {Q} \
	{ CAFF }

define_arc \
	-type non_seq_setup \
	-pinlist {CLK D PRE CLR M Q} \
	-ic "$VDD 0 $VDD $VDD 0 0" \
	-vector {1 0 F F X X} \
	-related_pin PRE \
	-pin CLR \
	-probe {Q M} \
	{ CAFF }

define_arc \
	-type non_seq_hold \
	-pinlist {CLK D PRE CLR M Q} \
	-ic "$VDD 0 $VDD $VDD 0 0" \
	-vector {1 0 F F X X} \
	-related_pin PRE \
	-pin CLR \
	-probe {Q M} \
	{ CAFF }

define_arc \
	-type setup \
	-pinlist {CLK D PRE CLR M Q} \
	-ic "0 $VDD 0 $VDD 0 0" \
	-vector {R 1 0 F X X} \
	-related_pin CLK \
	-pin CLR \
	-probe {Q M} \
	{ CAFF }

define_arc \
	-type hold \
	-pinlist {CLK D PRE CLR M Q} \
	-ic "0 $VDD 0 $VDD 0 0" \
	-vector {R 1 0 F X X} \
	-related_pin CLK \
	-pin CLR \
	-probe {Q M} \
	{ CAFF }

define_arc \
	-type setup \
	-pinlist {CLK D PRE CLR M Q} \
	-ic "0 $VDD 0 0 $VDD $VDD" \
	-vector {R F 0 0 X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ CAFF }

define_arc \
	-type hold \
	-pinlist {CLK D PRE CLR M Q} \
	-ic "0 $VDD 0 0 $VDD $VDD" \
	-vector {R F 0 0 X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ CAFF }

define_arc \
	-type setup \
	-pinlist {CLK D PRE CLR M Q} \
	-ic "0 0 0 0 0 0" \
	-vector {R R 0 0 X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ CAFF }

define_arc \
	-type hold \
	-pinlist {CLK D PRE CLR M Q} \
	-ic "0 0 0 0 0 0" \
	-vector {R R 0 0 X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ CAFF }

define_arc \
	-type setup \
	-pinlist {CLK D PRE CLR M Q} \
	-ic "0 0 $VDD 0 $VDD $VDD" \
	-vector {R 0 F 0 X X} \
	-related_pin CLK \
	-pin PRE \
	-probe {Q M} \
	{ CAFF }

define_arc \
	-type hold \
	-pinlist {CLK D PRE CLR M Q} \
	-ic "0 0 $VDD 0 $VDD $VDD" \
	-vector {R 0 F 0 X X} \
	-related_pin CLK \
	-pin PRE \
	-probe {Q M} \
	{ CAFF }

define_arc \
	-type combinational \
	-pinlist {CLK D Q} \
	-ic "$VDD 0 0" \
	-vector {1 R R} \
	-related_pin D \
	-pin Q \
	{ DLAT }

define_arc \
	-type combinational \
	-pinlist {CLK D Q} \
	-ic "$VDD $VDD $VDD" \
	-vector {1 F F} \
	-related_pin D \
	-pin Q \
	{ DLAT }

define_arc \
	-type edge \
	-pinlist {CLK D Q} \
	-ic "0 $VDD 0" \
	-vector {R 1 R} \
	-related_pin CLK \
	-pin Q \
	{ DLAT }

define_arc \
	-type edge \
	-pinlist {CLK D Q} \
	-ic "0 0 $VDD" \
	-vector {R 0 F} \
	-related_pin CLK \
	-pin Q \
	{ DLAT }

define_arc \
	-type hidden \
	-pinlist {CLK D Q} \
	-ic "$VDD 0 0" \
	-vector {F 0 0} \
	-pin CLK \
	{ DLAT }

define_arc \
	-type hidden \
	-pinlist {CLK D Q} \
	-ic "0 $VDD $VDD" \
	-vector {R 1 1} \
	-pin CLK \
	{ DLAT }

define_arc \
	-type hidden \
	-pinlist {CLK D Q} \
	-ic "0 $VDD $VDD" \
	-vector {0 F 1} \
	-pin D \
	{ DLAT }

define_arc \
	-type hidden \
	-pinlist {CLK D Q} \
	-ic "0 0 0" \
	-vector {0 R 0} \
	-pin D \
	{ DLAT }

define_leakage -when "CLK*!D*!Q" { DLAT }

define_leakage -when "CLK*D*Q" { DLAT }

define_leakage \
	-pinlist {CLK D Q} \
	-vector {0 1 1} \
	-when "!CLK*D*Q" \
	{ DLAT }

define_leakage \
	-pinlist {CLK D Q} \
	-vector {0 0 0} \
	-when "!CLK*!D*!Q" \
	{ DLAT }

define_leakage \
	-pinlist {CLK D Q} \
	-vector {0 1 0} \
	-when "!CLK*D*!Q" \
	{ DLAT }

define_leakage \
	-pinlist {CLK D Q} \
	-vector {0 0 1} \
	-when "!CLK*!D*Q" \
	{ DLAT }

define_arc \
	-type setup \
	-pinlist {CLK D Q} \
	-ic "$VDD $VDD $VDD" \
	-vector {F F X} \
	-related_pin CLK \
	-pin D \
	-probe {Q} \
	{ DLAT }

define_arc \
	-type hold \
	-pinlist {CLK D Q} \
	-ic "$VDD $VDD $VDD" \
	-vector {F F X} \
	-related_pin CLK \
	-pin D \
	-probe {Q} \
	{ DLAT }

define_arc \
	-type setup \
	-pinlist {CLK D Q} \
	-ic "$VDD 0 0" \
	-vector {F R X} \
	-related_pin CLK \
	-pin D \
	-probe {Q} \
	{ DLAT }

define_arc \
	-type hold \
	-pinlist {CLK D Q} \
	-ic "$VDD 0 0" \
	-vector {F R X} \
	-related_pin CLK \
	-pin D \
	-probe {Q} \
	{ DLAT }

define_arc \
	-type combinational \
	-pinlist {EN D Q} \
	-ic "$VDD 0 0" \
	-vector {1 R R} \
	-related_pin D \
	-pin Q \
	{ DLAT_EN }

define_arc \
	-type combinational \
	-pinlist {EN D Q} \
	-ic "$VDD $VDD $VDD" \
	-vector {1 F F} \
	-related_pin D \
	-pin Q \
	{ DLAT_EN }

define_arc \
	-type edge \
	-pinlist {EN D Q} \
	-ic "0 $VDD 0" \
	-vector {R 1 R} \
	-related_pin EN \
	-pin Q \
	{ DLAT_EN }

define_arc \
	-type edge \
	-pinlist {EN D Q} \
	-ic "0 0 $VDD" \
	-vector {R 0 F} \
	-related_pin EN \
	-pin Q \
	{ DLAT_EN }

define_arc \
	-type hidden \
	-pinlist {EN D Q} \
	-ic "$VDD 0 0" \
	-vector {F 0 0} \
	-pin EN \
	{ DLAT_EN }

define_arc \
	-type hidden \
	-pinlist {EN D Q} \
	-ic "0 0 0" \
	-vector {R 0 0} \
	-pin EN \
	{ DLAT_EN }

define_arc \
	-type hidden \
	-pinlist {EN D Q} \
	-ic "0 0 0" \
	-vector {0 R 0} \
	-pin D \
	{ DLAT_EN }

define_arc \
	-type hidden \
	-pinlist {EN D Q} \
	-ic "0 $VDD $VDD" \
	-vector {0 F 1} \
	-pin D \
	{ DLAT_EN }

define_leakage -when "!D*EN*!Q" { DLAT_EN }

define_leakage -when "D*EN*Q" { DLAT_EN }

define_leakage \
	-pinlist {EN D Q} \
	-vector {0 0 0} \
	-when "!D*!EN*!Q" \
	{ DLAT_EN }

define_leakage \
	-pinlist {EN D Q} \
	-vector {0 1 1} \
	-when "D*!EN*Q" \
	{ DLAT_EN }

define_leakage \
	-pinlist {EN D Q} \
	-vector {0 1 0} \
	-when "D*!EN*!Q" \
	{ DLAT_EN }

define_leakage \
	-pinlist {EN D Q} \
	-vector {0 0 1} \
	-when "!D*!EN*Q" \
	{ DLAT_EN }

define_arc \
	-type setup \
	-pinlist {EN D Q} \
	-ic "$VDD $VDD $VDD" \
	-vector {F F X} \
	-related_pin EN \
	-pin D \
	-probe {Q} \
	{ DLAT_EN }

define_arc \
	-type hold \
	-pinlist {EN D Q} \
	-ic "$VDD $VDD $VDD" \
	-vector {F F X} \
	-related_pin EN \
	-pin D \
	-probe {Q} \
	{ DLAT_EN }

define_arc \
	-type setup \
	-pinlist {EN D Q} \
	-ic "$VDD 0 0" \
	-vector {F R X} \
	-related_pin EN \
	-pin D \
	-probe {Q} \
	{ DLAT_EN }

define_arc \
	-type hold \
	-pinlist {EN D Q} \
	-ic "$VDD 0 0" \
	-vector {F R X} \
	-related_pin EN \
	-pin D \
	-probe {Q} \
	{ DLAT_EN }

define_arc \
	-type combinational \
	-pinlist {E D Q} \
	-ic "$VDD 0 0" \
	-vector {1 R R} \
	-related_pin D \
	-pin Q \
	{ DLAT_E }

define_arc \
	-type combinational \
	-pinlist {E D Q} \
	-ic "$VDD $VDD $VDD" \
	-vector {1 F F} \
	-related_pin D \
	-pin Q \
	{ DLAT_E }

define_arc \
	-type combinational \
	-pinlist {E D Q} \
	-ic "0 0 $VDD" \
	-vector {R 0 F} \
	-related_pin E \
	-pin Q \
	{ DLAT_E }

define_arc \
	-type combinational \
	-pinlist {E D Q} \
	-ic "0 $VDD 0" \
	-vector {R 1 R} \
	-related_pin E \
	-pin Q \
	{ DLAT_E }

define_arc \
	-type hidden \
	-pinlist {E D Q} \
	-ic "$VDD 0 0" \
	-vector {F 0 0} \
	-pin E \
	{ DLAT_E }

define_arc \
	-type hidden \
	-pinlist {E D Q} \
	-ic "0 $VDD $VDD" \
	-vector {R 1 1} \
	-pin E \
	{ DLAT_E }

define_arc \
	-type hidden \
	-pinlist {E D Q} \
	-ic "0 $VDD $VDD" \
	-vector {0 F 1} \
	-pin D \
	{ DLAT_E }

define_arc \
	-type hidden \
	-pinlist {E D Q} \
	-ic "0 0 0" \
	-vector {0 R 0} \
	-pin D \
	{ DLAT_E }

define_leakage -when "!D*E*!Q" { DLAT_E }

define_leakage -when "D*E*Q" { DLAT_E }

define_leakage \
	-pinlist {E D Q} \
	-vector {0 1 1} \
	-when "D*!E*Q" \
	{ DLAT_E }

define_leakage \
	-pinlist {E D Q} \
	-vector {0 0 0} \
	-when "!D*!E*!Q" \
	{ DLAT_E }

define_leakage \
	-pinlist {E D Q} \
	-vector {0 0 1} \
	-when "!D*!E*Q" \
	{ DLAT_E }

define_leakage \
	-pinlist {E D Q} \
	-vector {0 1 0} \
	-when "D*!E*!Q" \
	{ DLAT_E }

define_arc \
	-type non_seq_setup \
	-pinlist {E D Q} \
	-ic "$VDD $VDD $VDD" \
	-vector {F F X} \
	-related_pin E \
	-pin D \
	-probe {Q} \
	{ DLAT_E }

define_arc \
	-type non_seq_hold \
	-pinlist {E D Q} \
	-ic "$VDD $VDD $VDD" \
	-vector {F F X} \
	-related_pin E \
	-pin D \
	-probe {Q} \
	{ DLAT_E }

define_arc \
	-type non_seq_setup \
	-pinlist {E D Q} \
	-ic "$VDD 0 0" \
	-vector {F R X} \
	-related_pin E \
	-pin D \
	-probe {Q} \
	{ DLAT_E }

define_arc \
	-type non_seq_hold \
	-pinlist {E D Q} \
	-ic "$VDD 0 0" \
	-vector {F R X} \
	-related_pin E \
	-pin D \
	-probe {Q} \
	{ DLAT_E }

define_arc \
	-type hidden \
	-pinlist {CLK D Q} \
	-ic "$VDD $VDD $VDD" \
	-vector {F 1 1} \
	-pin CLK \
	{ GLAT }

define_arc \
	-type hidden \
	-pinlist {CLK D Q} \
	-ic "$VDD $VDD $VDD" \
	-vector {1 F 1} \
	-pin D \
	{ GLAT }

define_arc \
	-type hidden \
	-pinlist {CLK D Q} \
	-ic "$VDD 0 $VDD" \
	-vector {1 R 1} \
	-pin D \
	{ GLAT }

define_arc \
	-type hidden \
	-pinlist {CLK D Q} \
	-ic "0 $VDD $VDD" \
	-vector {R 1 1} \
	-pin CLK \
	{ GLAT }

define_leakage -when "CLK*D*Q" { GLAT }

define_leakage \
	-pinlist {CLK D Q} \
	-vector {1 0 1} \
	-when "CLK*!D*Q" \
	{ GLAT }

define_leakage \
	-pinlist {CLK D Q} \
	-vector {0 1 1} \
	-when "!CLK*D*Q" \
	{ GLAT }

define_leakage \
	-pinlist {CLK D Q} \
	-vector {0 0 1} \
	-when "!CLK*!D*Q" \
	{ GLAT }

define_arc \
	-type combinational \
	-pinlist {CLKA CLKB D Q} \
	-ic "0 $VDD 0 0" \
	-vector {0 1 R R} \
	-related_pin D \
	-pin Q \
	{ MUXLAT }

define_arc \
	-type combinational \
	-pinlist {CLKA CLKB D Q} \
	-ic "0 $VDD $VDD $VDD" \
	-vector {0 1 F F} \
	-related_pin D \
	-pin Q \
	{ MUXLAT }

define_arc \
	-type edge \
	-pinlist {CLKA CLKB D Q} \
	-ic "0 0 0 $VDD" \
	-vector {R 0 0 F} \
	-related_pin CLKA \
	-pin Q \
	{ MUXLAT }

define_arc \
	-type edge \
	-pinlist {CLKA CLKB D Q} \
	-ic "0 0 0 $VDD" \
	-vector {0 R 0 F} \
	-related_pin CLKB \
	-pin Q \
	{ MUXLAT }

define_arc \
	-type edge \
	-pinlist {CLKA CLKB D Q} \
	-ic "0 0 $VDD 0" \
	-vector {R 0 1 R} \
	-related_pin CLKA \
	-pin Q \
	{ MUXLAT }

define_arc \
	-type edge \
	-pinlist {CLKA CLKB D Q} \
	-ic "0 0 $VDD 0" \
	-vector {0 R 1 R} \
	-related_pin CLKB \
	-pin Q \
	{ MUXLAT }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB D Q} \
	-ic "0 $VDD 0 0" \
	-vector {R 1 0 0} \
	-pin CLKA \
	{ MUXLAT }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB D Q} \
	-ic "0 $VDD 0 0" \
	-vector {0 F 0 0} \
	-pin CLKB \
	{ MUXLAT }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB D Q} \
	-ic "$VDD 0 0 0" \
	-vector {F 0 0 0} \
	-pin CLKA \
	{ MUXLAT }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB D Q} \
	-ic "$VDD 0 0 0" \
	-vector {1 R 0 0} \
	-pin CLKB \
	{ MUXLAT }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB D Q} \
	-ic "0 0 0 0" \
	-vector {0 0 R 0} \
	-pin D \
	{ MUXLAT }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB D Q} \
	-ic "0 0 $VDD $VDD" \
	-vector {0 0 F 1} \
	-pin D \
	{ MUXLAT }

define_leakage -when "!CLKA*CLKB*!D*!Q" { MUXLAT }

define_leakage -when "!CLKA*CLKB*D*Q" { MUXLAT }

define_leakage -when "CLKA*!CLKB*!D*!Q" { MUXLAT }

define_leakage -when "CLKA*!CLKB*D*Q" { MUXLAT }

define_leakage -when "CLKA*CLKB*!D*!Q" { MUXLAT }

define_leakage -when "CLKA*CLKB*D*Q" { MUXLAT }

define_leakage \
	-pinlist {CLKA CLKB D Q} \
	-vector {0 0 0 0} \
	-when "!CLKA*!CLKB*!D*!Q" \
	{ MUXLAT }

define_leakage \
	-pinlist {CLKA CLKB D Q} \
	-vector {0 0 1 1} \
	-when "!CLKA*!CLKB*D*Q" \
	{ MUXLAT }

define_leakage \
	-pinlist {CLKA CLKB D Q} \
	-vector {0 0 0 1} \
	-when "!CLKA*!CLKB*!D*Q" \
	{ MUXLAT }

define_leakage \
	-pinlist {CLKA CLKB D Q} \
	-vector {0 0 1 0} \
	-when "!CLKA*!CLKB*D*!Q" \
	{ MUXLAT }

define_arc \
	-type setup \
	-pinlist {CLKA CLKB D Q} \
	-ic "$VDD 0 $VDD $VDD" \
	-vector {F 0 F X} \
	-related_pin CLKA \
	-pin D \
	-probe {Q} \
	{ MUXLAT }

define_arc \
	-type hold \
	-pinlist {CLKA CLKB D Q} \
	-ic "$VDD 0 $VDD $VDD" \
	-vector {F 0 F X} \
	-related_pin CLKA \
	-pin D \
	-probe {Q} \
	{ MUXLAT }

define_arc \
	-type setup \
	-pinlist {CLKA CLKB D Q} \
	-ic "$VDD 0 0 0" \
	-vector {F 0 R X} \
	-related_pin CLKA \
	-pin D \
	-probe {Q} \
	{ MUXLAT }

define_arc \
	-type hold \
	-pinlist {CLKA CLKB D Q} \
	-ic "$VDD 0 0 0" \
	-vector {F 0 R X} \
	-related_pin CLKA \
	-pin D \
	-probe {Q} \
	{ MUXLAT }

define_arc \
	-type setup \
	-pinlist {CLKA CLKB D Q} \
	-ic "0 $VDD $VDD $VDD" \
	-vector {0 F F X} \
	-related_pin CLKB \
	-pin D \
	-probe {Q} \
	{ MUXLAT }

define_arc \
	-type hold \
	-pinlist {CLKA CLKB D Q} \
	-ic "0 $VDD $VDD $VDD" \
	-vector {0 F F X} \
	-related_pin CLKB \
	-pin D \
	-probe {Q} \
	{ MUXLAT }

define_arc \
	-type setup \
	-pinlist {CLKA CLKB D Q} \
	-ic "0 $VDD 0 0" \
	-vector {0 F R X} \
	-related_pin CLKB \
	-pin D \
	-probe {Q} \
	{ MUXLAT }

define_arc \
	-type hold \
	-pinlist {CLKA CLKB D Q} \
	-ic "0 $VDD 0 0" \
	-vector {0 F R X} \
	-related_pin CLKB \
	-pin D \
	-probe {Q} \
	{ MUXLAT }

define_arc \
	-type combinational \
	-pinlist {CLKA CLKB D M Q} \
	-ic "0 $VDD 0 0 0" \
	-vector {0 1 R X R} \
	-related_pin D \
	-pin Q \
	{ MCDFF }

define_arc \
	-type combinational \
	-pinlist {CLKA CLKB D M Q} \
	-ic "0 $VDD $VDD $VDD $VDD" \
	-vector {0 1 F X F} \
	-related_pin D \
	-pin Q \
	{ MCDFF }

define_arc \
	-type edge \
	-pinlist {CLKA CLKB D M Q} \
	-ic "$VDD $VDD $VDD 0 0" \
	-vector {F 1 1 X R} \
	-related_pin CLKA \
	-pin Q \
	{ MCDFF }

define_arc \
	-type edge \
	-pinlist {CLKA CLKB D M Q} \
	-ic "0 0 0 0 $VDD" \
	-vector {0 R 0 X F} \
	-related_pin CLKB \
	-pin Q \
	{ MCDFF }

define_arc \
	-type edge \
	-pinlist {CLKA CLKB D M Q} \
	-ic "0 0 $VDD $VDD 0" \
	-vector {0 R 1 X R} \
	-related_pin CLKB \
	-pin Q \
	{ MCDFF }

define_arc \
	-type edge \
	-pinlist {CLKA CLKB D M Q} \
	-ic "$VDD $VDD 0 $VDD $VDD" \
	-vector {F 1 0 X F} \
	-related_pin CLKA \
	-pin Q \
	{ MCDFF }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB D M Q} \
	-ic "0 $VDD 0 0 0" \
	-vector {R 1 0 X 0} \
	-pin CLKA \
	{ MCDFF }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB D M Q} \
	-ic "0 $VDD 0 0 0" \
	-vector {0 F 0 X 0} \
	-pin CLKB \
	{ MCDFF }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB D M Q} \
	-ic "$VDD $VDD 0 0 0" \
	-vector {F 1 0 X 0} \
	-pin CLKA \
	{ MCDFF }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB D M Q} \
	-ic "$VDD $VDD 0 0 0" \
	-vector {1 1 R X 0} \
	-pin D \
	{ MCDFF }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB D M Q} \
	-ic "$VDD $VDD $VDD $VDD $VDD" \
	-vector {1 1 F X 1} \
	-pin D \
	{ MCDFF }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB D M Q} \
	-ic "0 0 $VDD $VDD $VDD" \
	-vector {0 R 1 X 1} \
	-pin CLKB \
	{ MCDFF }

define_leakage -when "!CLKA*CLKB*!D*!Q" { MCDFF }

define_leakage -when "!CLKA*CLKB*D*Q" { MCDFF }

define_leakage \
	-pinlist {CLKA CLKB D M Q} \
	-vector {1 1 0 0 0} \
	-when "CLKA*CLKB*!D*!Q" \
	{ MCDFF }

define_leakage \
	-pinlist {CLKA CLKB D M Q} \
	-vector {1 1 1 1 1} \
	-when "CLKA*CLKB*D*Q" \
	{ MCDFF }

define_leakage \
	-pinlist {CLKA CLKB D M Q} \
	-vector {0 0 1 1 1} \
	-when "!CLKA*!CLKB*D*Q" \
	{ MCDFF }

define_leakage \
	-pinlist {CLKA CLKB D M Q} \
	-vector {0 0 0 0 0} \
	-when "!CLKA*!CLKB*!D*!Q" \
	{ MCDFF }

define_leakage \
	-pinlist {CLKA CLKB D M Q} \
	-vector {1 1 1 0 0} \
	-when "CLKA*CLKB*D*!Q" \
	{ MCDFF }

define_leakage \
	-pinlist {CLKA CLKB D M Q} \
	-vector {1 0 0 0 0} \
	-when "CLKA*!CLKB*!D*!Q" \
	{ MCDFF }

define_leakage \
	-pinlist {CLKA CLKB D M Q} \
	-vector {0 0 0 0 1} \
	-when "!CLKA*!CLKB*!D*Q" \
	{ MCDFF }

define_leakage \
	-pinlist {CLKA CLKB D M Q} \
	-vector {0 0 1 1 0} \
	-when "!CLKA*!CLKB*D*!Q" \
	{ MCDFF }

define_leakage \
	-pinlist {CLKA CLKB D M Q} \
	-vector {1 0 1 1 1} \
	-when "CLKA*!CLKB*D*Q" \
	{ MCDFF }

define_leakage \
	-pinlist {CLKA CLKB D M Q} \
	-vector {1 1 0 1 1} \
	-when "CLKA*CLKB*!D*Q" \
	{ MCDFF }

define_leakage \
	-pinlist {CLKA CLKB D M Q} \
	-vector {1 0 1 0 0} \
	-when "CLKA*!CLKB*D*!Q" \
	{ MCDFF }

define_leakage \
	-pinlist {CLKA CLKB D M Q} \
	-vector {1 0 0 1 1} \
	-when "CLKA*!CLKB*!D*Q" \
	{ MCDFF }

define_leakage \
	-pinlist {CLKA CLKB D M Q} \
	-vector {1 0 1 1 0} \
	-when "CLKA*!CLKB*D*!Q" \
	{ MCDFF }

define_leakage \
	-pinlist {CLKA CLKB D M Q} \
	-vector {1 0 0 0 1} \
	-when "CLKA*!CLKB*!D*Q" \
	{ MCDFF }

define_leakage \
	-pinlist {CLKA CLKB D M Q} \
	-vector {1 0 0 1 0} \
	-when "CLKA*!CLKB*!D*!Q" \
	{ MCDFF }

define_leakage \
	-pinlist {CLKA CLKB D M Q} \
	-vector {1 0 1 0 1} \
	-when "CLKA*!CLKB*D*Q" \
	{ MCDFF }

define_arc \
	-type setup \
	-pinlist {CLKA CLKB D M Q} \
	-ic "0 0 $VDD $VDD $VDD" \
	-vector {R 0 F X X} \
	-related_pin CLKA \
	-pin D \
	-probe {M} \
	{ MCDFF }

define_arc \
	-type hold \
	-pinlist {CLKA CLKB D M Q} \
	-ic "0 0 $VDD $VDD $VDD" \
	-vector {R 0 F X X} \
	-related_pin CLKA \
	-pin D \
	-probe {M} \
	{ MCDFF }

define_arc \
	-type setup \
	-pinlist {CLKA CLKB D M Q} \
	-ic "0 $VDD $VDD $VDD $VDD" \
	-vector {R 1 F X X} \
	-related_pin CLKA \
	-pin D \
	-probe {Q M} \
	{ MCDFF }

define_arc \
	-type hold \
	-pinlist {CLKA CLKB D M Q} \
	-ic "0 $VDD $VDD $VDD $VDD" \
	-vector {R 1 F X X} \
	-related_pin CLKA \
	-pin D \
	-probe {Q M} \
	{ MCDFF }

define_arc \
	-type setup \
	-pinlist {CLKA CLKB D M Q} \
	-ic "0 0 0 0 0" \
	-vector {R 0 R X X} \
	-related_pin CLKA \
	-pin D \
	-probe {M} \
	{ MCDFF }

define_arc \
	-type hold \
	-pinlist {CLKA CLKB D M Q} \
	-ic "0 0 0 0 0" \
	-vector {R 0 R X X} \
	-related_pin CLKA \
	-pin D \
	-probe {M} \
	{ MCDFF }

define_arc \
	-type setup \
	-pinlist {CLKA CLKB D M Q} \
	-ic "0 $VDD 0 0 0" \
	-vector {R 1 R X X} \
	-related_pin CLKA \
	-pin D \
	-probe {Q M} \
	{ MCDFF }

define_arc \
	-type hold \
	-pinlist {CLKA CLKB D M Q} \
	-ic "0 $VDD 0 0 0" \
	-vector {R 1 R X X} \
	-related_pin CLKA \
	-pin D \
	-probe {Q M} \
	{ MCDFF }

define_arc \
	-type combinational \
	-pinlist {CLKA CLKB D XI7/m Q} \
	-ic "0 $VDD 0 0 0" \
	-vector {0 1 R X R} \
	-related_pin D \
	-pin Q \
	{ MCDFFX1 }

define_arc \
	-type combinational \
	-pinlist {CLKA CLKB D XI7/m Q} \
	-ic "0 $VDD $VDD $VDD $VDD" \
	-vector {0 1 F X F} \
	-related_pin D \
	-pin Q \
	{ MCDFFX1 }

define_arc \
	-type edge \
	-pinlist {CLKA CLKB D XI7/m Q} \
	-ic "$VDD $VDD 0 $VDD $VDD" \
	-vector {F 1 0 X F} \
	-related_pin CLKA \
	-pin Q \
	{ MCDFFX1 }

define_arc \
	-type edge \
	-pinlist {CLKA CLKB D XI7/m Q} \
	-ic "0 0 0 0 $VDD" \
	-vector {0 R 0 X F} \
	-related_pin CLKB \
	-pin Q \
	{ MCDFFX1 }

define_arc \
	-type edge \
	-pinlist {CLKA CLKB D XI7/m Q} \
	-ic "0 0 $VDD $VDD 0" \
	-vector {0 R 1 X R} \
	-related_pin CLKB \
	-pin Q \
	{ MCDFFX1 }

define_arc \
	-type edge \
	-pinlist {CLKA CLKB D XI7/m Q} \
	-ic "$VDD $VDD $VDD 0 0" \
	-vector {F 1 1 X R} \
	-related_pin CLKA \
	-pin Q \
	{ MCDFFX1 }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB D XI7/m Q} \
	-ic "0 $VDD 0 0 0" \
	-vector {R 1 0 X 0} \
	-pin CLKA \
	{ MCDFFX1 }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB D XI7/m Q} \
	-ic "0 $VDD 0 0 0" \
	-vector {0 F 0 X 0} \
	-pin CLKB \
	{ MCDFFX1 }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB D XI7/m Q} \
	-ic "$VDD $VDD 0 0 0" \
	-vector {F 1 0 X 0} \
	-pin CLKA \
	{ MCDFFX1 }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB D XI7/m Q} \
	-ic "$VDD $VDD 0 0 0" \
	-vector {1 1 R X 0} \
	-pin D \
	{ MCDFFX1 }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB D XI7/m Q} \
	-ic "$VDD $VDD $VDD $VDD $VDD" \
	-vector {1 1 F X 1} \
	-pin D \
	{ MCDFFX1 }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB D XI7/m Q} \
	-ic "0 0 $VDD $VDD $VDD" \
	-vector {0 R 1 X 1} \
	-pin CLKB \
	{ MCDFFX1 }

define_arc \
	-type combinational \
	-pinlist {CLKA CLKB D XI4/m Q} \
	-ic "0 $VDD 0 0 0" \
	-vector {0 1 R X R} \
	-related_pin D \
	-pin Q \
	{ MCDFFX4 }

define_arc \
	-type combinational \
	-pinlist {CLKA CLKB D XI4/m Q} \
	-ic "0 $VDD $VDD $VDD $VDD" \
	-vector {0 1 F X F} \
	-related_pin D \
	-pin Q \
	{ MCDFFX4 }

define_arc \
	-type edge \
	-pinlist {CLKA CLKB D XI4/m Q} \
	-ic "$VDD $VDD 0 $VDD $VDD" \
	-vector {F 1 0 X F} \
	-related_pin CLKA \
	-pin Q \
	{ MCDFFX4 }

define_arc \
	-type edge \
	-pinlist {CLKA CLKB D XI4/m Q} \
	-ic "0 0 0 0 $VDD" \
	-vector {0 R 0 X F} \
	-related_pin CLKB \
	-pin Q \
	{ MCDFFX4 }

define_arc \
	-type edge \
	-pinlist {CLKA CLKB D XI4/m Q} \
	-ic "0 0 $VDD $VDD 0" \
	-vector {0 R 1 X R} \
	-related_pin CLKB \
	-pin Q \
	{ MCDFFX4 }

define_arc \
	-type edge \
	-pinlist {CLKA CLKB D XI4/m Q} \
	-ic "$VDD $VDD $VDD 0 0" \
	-vector {F 1 1 X R} \
	-related_pin CLKA \
	-pin Q \
	{ MCDFFX4 }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB D XI4/m Q} \
	-ic "0 $VDD 0 0 0" \
	-vector {R 1 0 X 0} \
	-pin CLKA \
	{ MCDFFX4 }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB D XI4/m Q} \
	-ic "0 $VDD 0 0 0" \
	-vector {0 F 0 X 0} \
	-pin CLKB \
	{ MCDFFX4 }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB D XI4/m Q} \
	-ic "$VDD $VDD 0 0 0" \
	-vector {F 1 0 X 0} \
	-pin CLKA \
	{ MCDFFX4 }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB D XI4/m Q} \
	-ic "$VDD $VDD 0 0 0" \
	-vector {1 1 R X 0} \
	-pin D \
	{ MCDFFX4 }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB D XI4/m Q} \
	-ic "$VDD $VDD $VDD $VDD $VDD" \
	-vector {1 1 F X 1} \
	-pin D \
	{ MCDFFX4 }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB D XI4/m Q} \
	-ic "0 0 $VDD $VDD $VDD" \
	-vector {0 R 1 X 1} \
	-pin CLKB \
	{ MCDFFX4 }

define_leakage -when "!CLKA*CLKB*!D*!Q" { MCDFFX1 MCDFFX4 }

define_leakage -when "!CLKA*CLKB*D*Q" { MCDFFX1 MCDFFX4 }

define_leakage \
	-pinlist {CLKA CLKB D XI7/m Q} \
	-vector {1 1 0 0 0} \
	-when "CLKA*CLKB*!D*!Q" \
	{ MCDFFX1 }

define_leakage \
	-pinlist {CLKA CLKB D XI4/m Q} \
	-vector {1 1 0 0 0} \
	-when "CLKA*CLKB*!D*!Q" \
	{ MCDFFX4 }

define_leakage \
	-pinlist {CLKA CLKB D XI7/m Q} \
	-vector {1 1 1 1 1} \
	-when "CLKA*CLKB*D*Q" \
	{ MCDFFX1 }

define_leakage \
	-pinlist {CLKA CLKB D XI4/m Q} \
	-vector {1 1 1 1 1} \
	-when "CLKA*CLKB*D*Q" \
	{ MCDFFX4 }

define_leakage \
	-pinlist {CLKA CLKB D XI7/m Q} \
	-vector {0 0 1 1 1} \
	-when "!CLKA*!CLKB*D*Q" \
	{ MCDFFX1 }

define_leakage \
	-pinlist {CLKA CLKB D XI4/m Q} \
	-vector {0 0 1 1 1} \
	-when "!CLKA*!CLKB*D*Q" \
	{ MCDFFX4 }

define_leakage \
	-pinlist {CLKA CLKB D XI7/m Q} \
	-vector {0 0 0 0 0} \
	-when "!CLKA*!CLKB*!D*!Q" \
	{ MCDFFX1 }

define_leakage \
	-pinlist {CLKA CLKB D XI4/m Q} \
	-vector {0 0 0 0 0} \
	-when "!CLKA*!CLKB*!D*!Q" \
	{ MCDFFX4 }

define_leakage \
	-pinlist {CLKA CLKB D XI7/m Q} \
	-vector {1 1 0 1 1} \
	-when "CLKA*CLKB*!D*Q" \
	{ MCDFFX1 }

define_leakage \
	-pinlist {CLKA CLKB D XI4/m Q} \
	-vector {1 1 0 1 1} \
	-when "CLKA*CLKB*!D*Q" \
	{ MCDFFX4 }

define_leakage \
	-pinlist {CLKA CLKB D XI7/m Q} \
	-vector {0 0 0 0 1} \
	-when "!CLKA*!CLKB*!D*Q" \
	{ MCDFFX1 }

define_leakage \
	-pinlist {CLKA CLKB D XI4/m Q} \
	-vector {0 0 0 0 1} \
	-when "!CLKA*!CLKB*!D*Q" \
	{ MCDFFX4 }

define_leakage \
	-pinlist {CLKA CLKB D XI7/m Q} \
	-vector {1 0 0 0 0} \
	-when "CLKA*!CLKB*!D*!Q" \
	{ MCDFFX1 }

define_leakage \
	-pinlist {CLKA CLKB D XI4/m Q} \
	-vector {1 0 0 0 0} \
	-when "CLKA*!CLKB*!D*!Q" \
	{ MCDFFX4 }

define_leakage \
	-pinlist {CLKA CLKB D XI7/m Q} \
	-vector {0 0 1 1 0} \
	-when "!CLKA*!CLKB*D*!Q" \
	{ MCDFFX1 }

define_leakage \
	-pinlist {CLKA CLKB D XI4/m Q} \
	-vector {0 0 1 1 0} \
	-when "!CLKA*!CLKB*D*!Q" \
	{ MCDFFX4 }

define_leakage \
	-pinlist {CLKA CLKB D XI7/m Q} \
	-vector {1 0 1 1 1} \
	-when "CLKA*!CLKB*D*Q" \
	{ MCDFFX1 }

define_leakage \
	-pinlist {CLKA CLKB D XI4/m Q} \
	-vector {1 0 1 1 1} \
	-when "CLKA*!CLKB*D*Q" \
	{ MCDFFX4 }

define_leakage \
	-pinlist {CLKA CLKB D XI7/m Q} \
	-vector {1 1 1 0 0} \
	-when "CLKA*CLKB*D*!Q" \
	{ MCDFFX1 }

define_leakage \
	-pinlist {CLKA CLKB D XI4/m Q} \
	-vector {1 1 1 0 0} \
	-when "CLKA*CLKB*D*!Q" \
	{ MCDFFX4 }

define_leakage \
	-pinlist {CLKA CLKB D XI7/m Q} \
	-vector {1 0 0 1 1} \
	-when "CLKA*!CLKB*!D*Q" \
	{ MCDFFX1 }

define_leakage \
	-pinlist {CLKA CLKB D XI4/m Q} \
	-vector {1 0 0 1 1} \
	-when "CLKA*!CLKB*!D*Q" \
	{ MCDFFX4 }

define_leakage \
	-pinlist {CLKA CLKB D XI7/m Q} \
	-vector {1 0 1 0 0} \
	-when "CLKA*!CLKB*D*!Q" \
	{ MCDFFX1 }

define_leakage \
	-pinlist {CLKA CLKB D XI4/m Q} \
	-vector {1 0 1 0 0} \
	-when "CLKA*!CLKB*D*!Q" \
	{ MCDFFX4 }

define_leakage \
	-pinlist {CLKA CLKB D XI7/m Q} \
	-vector {1 0 0 0 1} \
	-when "CLKA*!CLKB*!D*Q" \
	{ MCDFFX1 }

define_leakage \
	-pinlist {CLKA CLKB D XI4/m Q} \
	-vector {1 0 0 0 1} \
	-when "CLKA*!CLKB*!D*Q" \
	{ MCDFFX4 }

define_leakage \
	-pinlist {CLKA CLKB D XI7/m Q} \
	-vector {1 0 1 1 0} \
	-when "CLKA*!CLKB*D*!Q" \
	{ MCDFFX1 }

define_leakage \
	-pinlist {CLKA CLKB D XI4/m Q} \
	-vector {1 0 1 1 0} \
	-when "CLKA*!CLKB*D*!Q" \
	{ MCDFFX4 }

define_leakage \
	-pinlist {CLKA CLKB D XI7/m Q} \
	-vector {1 0 0 1 0} \
	-when "CLKA*!CLKB*!D*!Q" \
	{ MCDFFX1 }

define_leakage \
	-pinlist {CLKA CLKB D XI4/m Q} \
	-vector {1 0 0 1 0} \
	-when "CLKA*!CLKB*!D*!Q" \
	{ MCDFFX4 }

define_leakage \
	-pinlist {CLKA CLKB D XI7/m Q} \
	-vector {1 0 1 0 1} \
	-when "CLKA*!CLKB*D*Q" \
	{ MCDFFX1 }

define_leakage \
	-pinlist {CLKA CLKB D XI4/m Q} \
	-vector {1 0 1 0 1} \
	-when "CLKA*!CLKB*D*Q" \
	{ MCDFFX4 }

define_arc \
	-type setup \
	-pinlist {CLKA CLKB D XI7/m Q} \
	-ic "0 $VDD $VDD $VDD $VDD" \
	-vector {R 1 F X X} \
	-related_pin CLKA \
	-pin D \
	-probe {Q XI7/m} \
	{ MCDFFX1 }

define_arc \
	-type hold \
	-pinlist {CLKA CLKB D XI7/m Q} \
	-ic "0 $VDD $VDD $VDD $VDD" \
	-vector {R 1 F X X} \
	-related_pin CLKA \
	-pin D \
	-probe {Q XI7/m} \
	{ MCDFFX1 }

define_arc \
	-type setup \
	-pinlist {CLKA CLKB D XI4/m Q} \
	-ic "0 $VDD $VDD $VDD $VDD" \
	-vector {R 1 F X X} \
	-related_pin CLKA \
	-pin D \
	-probe {Q XI4/m} \
	{ MCDFFX4 }

define_arc \
	-type hold \
	-pinlist {CLKA CLKB D XI4/m Q} \
	-ic "0 $VDD $VDD $VDD $VDD" \
	-vector {R 1 F X X} \
	-related_pin CLKA \
	-pin D \
	-probe {Q XI4/m} \
	{ MCDFFX4 }

define_arc \
	-type setup \
	-pinlist {CLKA CLKB D XI7/m Q} \
	-ic "0 0 $VDD $VDD $VDD" \
	-vector {R 0 F X X} \
	-related_pin CLKA \
	-pin D \
	-probe {XI7/m} \
	{ MCDFFX1 }

define_arc \
	-type hold \
	-pinlist {CLKA CLKB D XI7/m Q} \
	-ic "0 0 $VDD $VDD $VDD" \
	-vector {R 0 F X X} \
	-related_pin CLKA \
	-pin D \
	-probe {XI7/m} \
	{ MCDFFX1 }

define_arc \
	-type setup \
	-pinlist {CLKA CLKB D XI4/m Q} \
	-ic "0 0 $VDD $VDD $VDD" \
	-vector {R 0 F X X} \
	-related_pin CLKA \
	-pin D \
	-probe {XI4/m} \
	{ MCDFFX4 }

define_arc \
	-type hold \
	-pinlist {CLKA CLKB D XI4/m Q} \
	-ic "0 0 $VDD $VDD $VDD" \
	-vector {R 0 F X X} \
	-related_pin CLKA \
	-pin D \
	-probe {XI4/m} \
	{ MCDFFX4 }

define_arc \
	-type setup \
	-pinlist {CLKA CLKB D XI7/m Q} \
	-ic "0 $VDD 0 0 0" \
	-vector {R 1 R X X} \
	-related_pin CLKA \
	-pin D \
	-probe {Q XI7/m} \
	{ MCDFFX1 }

define_arc \
	-type hold \
	-pinlist {CLKA CLKB D XI7/m Q} \
	-ic "0 $VDD 0 0 0" \
	-vector {R 1 R X X} \
	-related_pin CLKA \
	-pin D \
	-probe {Q XI7/m} \
	{ MCDFFX1 }

define_arc \
	-type setup \
	-pinlist {CLKA CLKB D XI4/m Q} \
	-ic "0 $VDD 0 0 0" \
	-vector {R 1 R X X} \
	-related_pin CLKA \
	-pin D \
	-probe {Q XI4/m} \
	{ MCDFFX4 }

define_arc \
	-type hold \
	-pinlist {CLKA CLKB D XI4/m Q} \
	-ic "0 $VDD 0 0 0" \
	-vector {R 1 R X X} \
	-related_pin CLKA \
	-pin D \
	-probe {Q XI4/m} \
	{ MCDFFX4 }

define_arc \
	-type setup \
	-pinlist {CLKA CLKB D XI7/m Q} \
	-ic "0 0 0 0 0" \
	-vector {R 0 R X X} \
	-related_pin CLKA \
	-pin D \
	-probe {XI7/m} \
	{ MCDFFX1 }

define_arc \
	-type hold \
	-pinlist {CLKA CLKB D XI7/m Q} \
	-ic "0 0 0 0 0" \
	-vector {R 0 R X X} \
	-related_pin CLKA \
	-pin D \
	-probe {XI7/m} \
	{ MCDFFX1 }

define_arc \
	-type setup \
	-pinlist {CLKA CLKB D XI4/m Q} \
	-ic "0 0 0 0 0" \
	-vector {R 0 R X X} \
	-related_pin CLKA \
	-pin D \
	-probe {XI4/m} \
	{ MCDFFX4 }

define_arc \
	-type hold \
	-pinlist {CLKA CLKB D XI4/m Q} \
	-ic "0 0 0 0 0" \
	-vector {R 0 R X X} \
	-related_pin CLKA \
	-pin D \
	-probe {XI4/m} \
	{ MCDFFX4 }

define_arc \
	-type combinational \
	-pinlist {CLK D Q} \
	-ic "0 0 0" \
	-vector {0 R R} \
	-related_pin D \
	-pin Q \
	{ TCASC }

define_arc \
	-type combinational \
	-pinlist {CLK D Q} \
	-ic "0 $VDD $VDD" \
	-vector {0 F F} \
	-related_pin D \
	-pin Q \
	{ TCASC }

define_arc \
	-type edge \
	-pinlist {CLK D Q} \
	-ic "$VDD 0 $VDD" \
	-vector {F 0 F} \
	-related_pin CLK \
	-pin Q \
	{ TCASC }

define_arc \
	-type edge \
	-pinlist {CLK D Q} \
	-ic "$VDD $VDD 0" \
	-vector {F 1 R} \
	-related_pin CLK \
	-pin Q \
	{ TCASC }

define_arc \
	-type hidden \
	-pinlist {CLK D Q} \
	-ic "0 0 0" \
	-vector {R 0 0} \
	-pin CLK \
	{ TCASC }

define_arc \
	-type hidden \
	-pinlist {CLK D Q} \
	-ic "$VDD 0 0" \
	-vector {F 0 0} \
	-pin CLK \
	{ TCASC }

define_arc \
	-type hidden \
	-pinlist {CLK D Q} \
	-ic "$VDD 0 0" \
	-vector {1 R 0} \
	-pin D \
	{ TCASC }

define_arc \
	-type hidden \
	-pinlist {CLK D Q} \
	-ic "$VDD $VDD $VDD" \
	-vector {1 F 1} \
	-pin D \
	{ TCASC }

define_leakage -when "!CLK*!D*!Q" { TCASC }

define_leakage -when "!CLK*D*Q" { TCASC }

define_leakage \
	-pinlist {CLK D Q} \
	-vector {1 0 0} \
	-when "CLK*!D*!Q" \
	{ TCASC }

define_leakage \
	-pinlist {CLK D Q} \
	-vector {1 1 1} \
	-when "CLK*D*Q" \
	{ TCASC }

define_leakage \
	-pinlist {CLK D Q} \
	-vector {1 0 1} \
	-when "CLK*!D*Q" \
	{ TCASC }

define_leakage \
	-pinlist {CLK D Q} \
	-vector {1 1 0} \
	-when "CLK*D*!Q" \
	{ TCASC }

define_arc \
	-type setup \
	-pinlist {CLK D M Q} \
	-ic "0 $VDD $VDD $VDD" \
	-vector {R F X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ TCASC }

define_arc \
	-type hold \
	-pinlist {CLK D M Q} \
	-ic "0 $VDD $VDD $VDD" \
	-vector {R F X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ TCASC }

define_arc \
	-type setup \
	-pinlist {CLK D M Q} \
	-ic "0 0 0 0" \
	-vector {R R X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ TCASC }

define_arc \
	-type hold \
	-pinlist {CLK D M Q} \
	-ic "0 0 0 0" \
	-vector {R R X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ TCASC }

define_arc \
	-type combinational \
	-pinlist {CLK D T} \
	-ic "$VDD 0 0" \
	-vector {1 R R} \
	-related_pin D \
	-pin T \
	{ XLAT }

define_arc \
	-type combinational \
	-pinlist {CLK D T} \
	-ic "$VDD $VDD 0" \
	-vector {1 F R} \
	-related_pin D \
	-pin T \
	{ XLAT }

define_arc \
	-type edge \
	-pinlist {CLK D T} \
	-ic "$VDD $VDD $VDD" \
	-vector {F 1 F} \
	-related_pin CLK \
	-pin T \
	{ XLAT }

define_arc \
	-type combinational \
	-pinlist {CLK D T} \
	-ic "$VDD $VDD $VDD" \
	-vector {1 F F} \
	-related_pin D \
	-pin T \
	{ XLAT }

define_arc \
	-type edge \
	-pinlist {CLK D T} \
	-ic "0 $VDD $VDD" \
	-vector {R 1 F} \
	-related_pin CLK \
	-pin T \
	{ XLAT }

define_arc \
	-type combinational \
	-pinlist {CLK D T} \
	-ic "$VDD 0 $VDD" \
	-vector {1 R F} \
	-related_pin D \
	-pin T \
	{ XLAT }

define_arc \
	-type hidden \
	-pinlist {CLK D T} \
	-ic "$VDD 0 0" \
	-vector {F 0 0} \
	-pin CLK \
	{ XLAT }

define_arc \
	-type hidden \
	-pinlist {CLK D T} \
	-ic "0 0 0" \
	-vector {R 0 0} \
	-pin CLK \
	{ XLAT }

define_leakage \
	-pinlist {CLK D T} \
	-vector {1 0 0} \
	-when "CLK*!D*!T" \
	{ XLAT }

define_leakage \
	-pinlist {CLK D T} \
	-vector {0 0 0} \
	-when "!CLK*!D*!T" \
	{ XLAT }

define_leakage \
	-pinlist {CLK D T} \
	-vector {1 1 0} \
	-when "CLK*D*!T" \
	{ XLAT }

define_leakage \
	-pinlist {CLK D T} \
	-vector {0 1 0} \
	-when "!CLK*D*!T" \
	{ XLAT }

define_leakage \
	-pinlist {CLK D T} \
	-vector {1 1 1} \
	-when "CLK*D*T" \
	{ XLAT }

define_leakage \
	-pinlist {CLK D T} \
	-vector {0 1 1} \
	-when "!CLK*D*T" \
	{ XLAT }

define_leakage \
	-pinlist {CLK D T} \
	-vector {1 0 1} \
	-when "CLK*!D*T" \
	{ XLAT }

define_leakage \
	-pinlist {CLK D T} \
	-vector {0 0 1} \
	-when "!CLK*!D*T" \
	{ XLAT }

define_arc \
	-type setup \
	-pinlist {CLK D M2 T} \
	-ic "$VDD $VDD $VDD 0" \
	-vector {F F X X} \
	-related_pin CLK \
	-pin D \
	-probe {M2} \
	{ XLAT }

define_arc \
	-type hold \
	-pinlist {CLK D M2 T} \
	-ic "$VDD $VDD $VDD 0" \
	-vector {F F X X} \
	-related_pin CLK \
	-pin D \
	-probe {M2} \
	{ XLAT }

define_arc \
	-type setup \
	-pinlist {CLK D M2 T} \
	-ic "$VDD 0 0 0" \
	-vector {F R X X} \
	-related_pin CLK \
	-pin D \
	-probe {M2} \
	{ XLAT }

define_arc \
	-type hold \
	-pinlist {CLK D M2 T} \
	-ic "$VDD 0 0 0" \
	-vector {F R X X} \
	-related_pin CLK \
	-pin D \
	-probe {M2} \
	{ XLAT }

define_arc \
	-type setup \
	-pinlist {CLK D M T} \
	-ic "0 $VDD $VDD 0" \
	-vector {R F X X} \
	-related_pin CLK \
	-pin D \
	-probe {M} \
	{ XLAT }

define_arc \
	-type hold \
	-pinlist {CLK D M T} \
	-ic "0 $VDD $VDD 0" \
	-vector {R F X X} \
	-related_pin CLK \
	-pin D \
	-probe {M} \
	{ XLAT }

define_arc \
	-type setup \
	-pinlist {CLK D M T} \
	-ic "0 0 0 0" \
	-vector {R R X X} \
	-related_pin CLK \
	-pin D \
	-probe {M} \
	{ XLAT }

define_arc \
	-type hold \
	-pinlist {CLK D M T} \
	-ic "0 0 0 0" \
	-vector {R R X X} \
	-related_pin CLK \
	-pin D \
	-probe {M} \
	{ XLAT }

define_arc \
	-type edge \
	-pinlist {CLKA CLKB D M1 M2 Q} \
	-ic "0 0 $VDD $VDD 0 0" \
	-vector {R 0 1 X X R} \
	-related_pin CLKA \
	-pin Q \
	{ HPIPE }

define_arc \
	-type edge \
	-pinlist {CLKA CLKB D M1 M2 Q} \
	-ic "0 0 0 0 $VDD $VDD" \
	-vector {R 0 0 X X F} \
	-related_pin CLKA \
	-pin Q \
	{ HPIPE }

define_arc \
	-type edge \
	-pinlist {CLKA CLKB D M1 M2 Q} \
	-ic "$VDD $VDD $VDD $VDD $VDD 0" \
	-vector {1 F 1 X X R} \
	-related_pin CLKB \
	-pin Q \
	{ HPIPE }

define_arc \
	-type edge \
	-pinlist {CLKA CLKB D M1 M2 Q} \
	-ic "$VDD $VDD 0 0 0 $VDD" \
	-vector {1 F 0 X X F} \
	-related_pin CLKB \
	-pin Q \
	{ HPIPE }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB D M1 M2 Q} \
	-ic "$VDD 0 0 0 0 0" \
	-vector {F 0 0 X X 0} \
	-pin CLKA \
	{ HPIPE }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB D M1 M2 Q} \
	-ic "$VDD 0 0 0 0 0" \
	-vector {1 R 0 X X 0} \
	-pin CLKB \
	{ HPIPE }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB D M1 M2 Q} \
	-ic "$VDD 0 0 0 0 0" \
	-vector {1 0 R X X 0} \
	-pin D \
	{ HPIPE }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB D M1 M2 Q} \
	-ic "$VDD 0 $VDD $VDD $VDD $VDD" \
	-vector {1 0 F X X 1} \
	-pin D \
	{ HPIPE }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB D M1 M2 Q} \
	-ic "0 0 $VDD $VDD $VDD $VDD" \
	-vector {R 0 1 X X 1} \
	-pin CLKA \
	{ HPIPE }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB D M1 M2 Q} \
	-ic "$VDD $VDD 0 0 0 0" \
	-vector {1 F 0 X X 0} \
	-pin CLKB \
	{ HPIPE }

define_leakage \
	-pinlist {CLKA CLKB D M1 M2 Q} \
	-vector {1 0 0 0 0 0} \
	-when "CLKA*!CLKB*!D*!Q" \
	{ HPIPE }

define_leakage \
	-pinlist {CLKA CLKB D M1 M2 Q} \
	-vector {1 0 1 1 1 1} \
	-when "CLKA*!CLKB*D*Q" \
	{ HPIPE }

define_leakage \
	-pinlist {CLKA CLKB D M1 M2 Q} \
	-vector {1 0 0 1 1 1} \
	-when "CLKA*!CLKB*!D*Q" \
	{ HPIPE }

define_leakage \
	-pinlist {CLKA CLKB D M1 M2 Q} \
	-vector {0 0 1 1 1 1} \
	-when "!CLKA*!CLKB*D*Q" \
	{ HPIPE }

define_leakage \
	-pinlist {CLKA CLKB D M1 M2 Q} \
	-vector {1 1 0 0 0 0} \
	-when "CLKA*CLKB*!D*!Q" \
	{ HPIPE }

define_leakage \
	-pinlist {CLKA CLKB D M1 M2 Q} \
	-vector {0 0 0 0 0 0} \
	-when "!CLKA*!CLKB*!D*!Q" \
	{ HPIPE }

define_leakage \
	-pinlist {CLKA CLKB D M1 M2 Q} \
	-vector {1 0 1 0 0 0} \
	-when "CLKA*!CLKB*D*!Q" \
	{ HPIPE }

define_leakage \
	-pinlist {CLKA CLKB D M1 M2 Q} \
	-vector {1 1 1 1 1 1} \
	-when "CLKA*CLKB*D*Q" \
	{ HPIPE }

define_leakage \
	-pinlist {CLKA CLKB D M1 M2 Q} \
	-vector {0 1 1 1 1 1} \
	-when "!CLKA*CLKB*D*Q" \
	{ HPIPE }

define_leakage \
	-pinlist {CLKA CLKB D M1 M2 Q} \
	-vector {0 1 0 0 0 0} \
	-when "!CLKA*CLKB*!D*!Q" \
	{ HPIPE }

define_leakage \
	-pinlist {CLKA CLKB D M1 M2 Q} \
	-vector {1 1 1 0 0 0} \
	-when "CLKA*CLKB*D*!Q" \
	{ HPIPE }

define_leakage \
	-pinlist {CLKA CLKB D M1 M2 Q} \
	-vector {0 0 1 1 0 0} \
	-when "!CLKA*!CLKB*D*!Q" \
	{ HPIPE }

define_leakage \
	-pinlist {CLKA CLKB D M1 M2 Q} \
	-vector {1 1 0 1 1 1} \
	-when "CLKA*CLKB*!D*Q" \
	{ HPIPE }

define_leakage \
	-pinlist {CLKA CLKB D M1 M2 Q} \
	-vector {0 0 0 0 1 1} \
	-when "!CLKA*!CLKB*!D*Q" \
	{ HPIPE }

define_leakage \
	-pinlist {CLKA CLKB D M1 M2 Q} \
	-vector {0 1 0 0 1 1} \
	-when "!CLKA*CLKB*!D*Q" \
	{ HPIPE }

define_leakage \
	-pinlist {CLKA CLKB D M1 M2 Q} \
	-vector {0 1 1 1 0 0} \
	-when "!CLKA*CLKB*D*!Q" \
	{ HPIPE }

define_leakage \
	-pinlist {CLKA CLKB D M1 M2 Q} \
	-vector {1 1 1 1 1 0} \
	-when "CLKA*CLKB*D*!Q" \
	{ HPIPE }

define_leakage \
	-pinlist {CLKA CLKB D M1 M2 Q} \
	-vector {1 1 0 0 0 1} \
	-when "CLKA*CLKB*!D*Q" \
	{ HPIPE }

define_leakage \
	-pinlist {CLKA CLKB D M1 M2 Q} \
	-vector {1 1 0 1 1 0} \
	-when "CLKA*CLKB*!D*!Q" \
	{ HPIPE }

define_leakage \
	-pinlist {CLKA CLKB D M1 M2 Q} \
	-vector {0 1 0 0 0 1} \
	-when "!CLKA*CLKB*!D*Q" \
	{ HPIPE }

define_leakage \
	-pinlist {CLKA CLKB D M1 M2 Q} \
	-vector {1 1 1 0 0 1} \
	-when "CLKA*CLKB*D*Q" \
	{ HPIPE }

define_leakage \
	-pinlist {CLKA CLKB D M1 M2 Q} \
	-vector {0 1 1 1 1 0} \
	-when "!CLKA*CLKB*D*!Q" \
	{ HPIPE }

define_leakage \
	-pinlist {CLKA CLKB D M1 M2 Q} \
	-vector {0 1 1 1 0 1} \
	-when "!CLKA*CLKB*D*Q" \
	{ HPIPE }

define_leakage \
	-pinlist {CLKA CLKB D M1 M2 Q} \
	-vector {0 1 0 0 1 0} \
	-when "!CLKA*CLKB*!D*!Q" \
	{ HPIPE }

define_arc \
	-type setup \
	-pinlist {CLKA CLKB D M1 M2 Q} \
	-ic "0 $VDD $VDD $VDD $VDD $VDD" \
	-vector {R 1 F X X X} \
	-related_pin CLKA \
	-pin D \
	-probe {M1 M2} \
	{ HPIPE }

define_arc \
	-type hold \
	-pinlist {CLKA CLKB D M1 M2 Q} \
	-ic "0 $VDD $VDD $VDD $VDD $VDD" \
	-vector {R 1 F X X X} \
	-related_pin CLKA \
	-pin D \
	-probe {M1 M2} \
	{ HPIPE }

define_arc \
	-type setup \
	-pinlist {CLKA CLKB D M1 M2 Q} \
	-ic "0 0 $VDD $VDD $VDD $VDD" \
	-vector {R 0 F X X X} \
	-related_pin CLKA \
	-pin D \
	-probe {Q M1 M2} \
	{ HPIPE }

define_arc \
	-type hold \
	-pinlist {CLKA CLKB D M1 M2 Q} \
	-ic "0 0 $VDD $VDD $VDD $VDD" \
	-vector {R 0 F X X X} \
	-related_pin CLKA \
	-pin D \
	-probe {Q M1 M2} \
	{ HPIPE }

define_arc \
	-type setup \
	-pinlist {CLKA CLKB D M1 M2 Q} \
	-ic "0 $VDD 0 0 0 0" \
	-vector {R 1 R X X X} \
	-related_pin CLKA \
	-pin D \
	-probe {M1 M2} \
	{ HPIPE }

define_arc \
	-type hold \
	-pinlist {CLKA CLKB D M1 M2 Q} \
	-ic "0 $VDD 0 0 0 0" \
	-vector {R 1 R X X X} \
	-related_pin CLKA \
	-pin D \
	-probe {M1 M2} \
	{ HPIPE }

define_arc \
	-type setup \
	-pinlist {CLKA CLKB D M1 M2 Q} \
	-ic "0 0 0 0 0 0" \
	-vector {R 0 R X X X} \
	-related_pin CLKA \
	-pin D \
	-probe {Q M1 M2} \
	{ HPIPE }

define_arc \
	-type hold \
	-pinlist {CLKA CLKB D M1 M2 Q} \
	-ic "0 0 0 0 0 0" \
	-vector {R 0 R X X X} \
	-related_pin CLKA \
	-pin D \
	-probe {Q M1 M2} \
	{ HPIPE }

define_arc \
	-type edge \
	-pinlist {CLKA CLKB DA DB MA MB Q} \
	-ic "0 $VDD $VDD 0 $VDD 0 0" \
	-vector {R 1 1 0 X X R} \
	-related_pin CLKA \
	-pin Q \
	{ DCMUX }

define_arc \
	-type edge \
	-pinlist {CLKA CLKB DA DB MA MB Q} \
	-ic "$VDD 0 0 $VDD 0 $VDD 0" \
	-vector {1 R 0 1 X X R} \
	-related_pin CLKB \
	-pin Q \
	{ DCMUX }

define_arc \
	-type edge \
	-pinlist {CLKA CLKB DA DB MA MB Q} \
	-ic "0 0 0 $VDD 0 $VDD $VDD" \
	-vector {R 0 0 1 X X F} \
	-related_pin CLKA \
	-pin Q \
	{ DCMUX }

define_arc \
	-type edge \
	-pinlist {CLKA CLKB DA DB MA MB Q} \
	-ic "0 0 $VDD 0 $VDD 0 $VDD" \
	-vector {0 R 1 0 X X F} \
	-related_pin CLKB \
	-pin Q \
	{ DCMUX }

define_arc \
	-type combinational \
	-pinlist {CLKA CLKB DA DB MA MB Q} \
	-ic "$VDD $VDD $VDD 0 $VDD 0 $VDD" \
	-vector {F 1 1 0 X X F} \
	-related_pin CLKA \
	-pin Q \
	{ DCMUX }

define_arc \
	-type combinational \
	-pinlist {CLKA CLKB DA DB MA MB Q} \
	-ic "$VDD $VDD 0 $VDD 0 $VDD $VDD" \
	-vector {1 F 0 1 X X F} \
	-related_pin CLKB \
	-pin Q \
	{ DCMUX }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB DA DB MA MB Q} \
	-ic "0 $VDD $VDD 0 $VDD 0 0" \
	-vector {0 F 1 0 X X 0} \
	-pin CLKB \
	{ DCMUX }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB DA DB MA MB Q} \
	-ic "0 $VDD $VDD 0 $VDD 0 0" \
	-vector {0 1 F 0 X X 0} \
	-pin DA \
	{ DCMUX }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB DA DB MA MB Q} \
	-ic "0 $VDD $VDD 0 $VDD 0 0" \
	-vector {0 1 1 R X X 0} \
	-pin DB \
	{ DCMUX }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB DA DB MA MB Q} \
	-ic "$VDD 0 $VDD $VDD $VDD $VDD $VDD" \
	-vector {F 0 1 1 X X 1} \
	-pin CLKA \
	{ DCMUX }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB DA DB MA MB Q} \
	-ic "$VDD 0 $VDD $VDD $VDD $VDD $VDD" \
	-vector {1 R 1 1 X X 1} \
	-pin CLKB \
	{ DCMUX }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB DA DB MA MB Q} \
	-ic "$VDD 0 $VDD $VDD $VDD $VDD $VDD" \
	-vector {1 0 1 F X X 1} \
	-pin DB \
	{ DCMUX }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB DA DB MA MB Q} \
	-ic "0 $VDD 0 $VDD 0 $VDD $VDD" \
	-vector {R 1 0 1 X X 1} \
	-pin CLKA \
	{ DCMUX }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB DA DB MA MB Q} \
	-ic "0 $VDD 0 $VDD 0 $VDD $VDD" \
	-vector {0 1 R 1 X X 1} \
	-pin DA \
	{ DCMUX }

define_leakage \
	-pinlist {CLKA CLKB DA DB MA MB Q} \
	-vector {0 1 1 0 1 0 0} \
	-when "!CLKA*CLKB*DA*!DB*!Q" \
	{ DCMUX }

define_leakage \
	-pinlist {CLKA CLKB DA DB MA MB Q} \
	-vector {1 0 1 1 1 1 1} \
	-when "CLKA*!CLKB*DA*DB*Q" \
	{ DCMUX }

define_leakage \
	-pinlist {CLKA CLKB DA DB MA MB Q} \
	-vector {0 1 0 1 0 1 1} \
	-when "!CLKA*CLKB*!DA*DB*Q" \
	{ DCMUX }

define_leakage \
	-pinlist {CLKA CLKB DA DB MA MB Q} \
	-vector {0 1 1 1 1 1 1} \
	-when "!CLKA*CLKB*DA*DB*Q" \
	{ DCMUX }

define_leakage \
	-pinlist {CLKA CLKB DA DB MA MB Q} \
	-vector {1 0 0 1 0 1 0} \
	-when "CLKA*!CLKB*!DA*DB*!Q" \
	{ DCMUX }

define_leakage \
	-pinlist {CLKA CLKB DA DB MA MB Q} \
	-vector {1 0 1 0 1 0 1} \
	-when "CLKA*!CLKB*DA*!DB*Q" \
	{ DCMUX }

define_leakage \
	-pinlist {CLKA CLKB DA DB MA MB Q} \
	-vector {0 1 0 0 0 0 0} \
	-when "!CLKA*CLKB*!DA*!DB*!Q" \
	{ DCMUX }

define_leakage \
	-pinlist {CLKA CLKB DA DB MA MB Q} \
	-vector {1 0 0 0 0 0 0} \
	-when "CLKA*!CLKB*!DA*!DB*!Q" \
	{ DCMUX }

define_leakage \
	-pinlist {CLKA CLKB DA DB MA MB Q} \
	-vector {1 1 0 0 0 0 0} \
	-when "CLKA*CLKB*!DA*!DB*!Q" \
	{ DCMUX }

define_leakage \
	-pinlist {CLKA CLKB DA DB MA MB Q} \
	-vector {1 0 1 0 0 0 0} \
	-when "CLKA*!CLKB*DA*!DB*!Q" \
	{ DCMUX }

define_leakage \
	-pinlist {CLKA CLKB DA DB MA MB Q} \
	-vector {0 0 0 1 0 1 1} \
	-when "!CLKA*!CLKB*!DA*DB*Q" \
	{ DCMUX }

define_leakage \
	-pinlist {CLKA CLKB DA DB MA MB Q} \
	-vector {0 1 0 1 0 0 0} \
	-when "!CLKA*CLKB*!DA*DB*!Q" \
	{ DCMUX }

define_leakage \
	-pinlist {CLKA CLKB DA DB MA MB Q} \
	-vector {0 0 1 0 1 0 1} \
	-when "!CLKA*!CLKB*DA*!DB*Q" \
	{ DCMUX }

define_leakage \
	-pinlist {CLKA CLKB DA DB MA MB Q} \
	-vector {1 1 1 0 1 0 1} \
	-when "CLKA*CLKB*DA*!DB*Q" \
	{ DCMUX }

define_leakage \
	-pinlist {CLKA CLKB DA DB MA MB Q} \
	-vector {0 1 1 0 1 1 1} \
	-when "!CLKA*CLKB*DA*!DB*Q" \
	{ DCMUX }

define_leakage \
	-pinlist {CLKA CLKB DA DB MA MB Q} \
	-vector {0 0 1 0 1 0 0} \
	-when "!CLKA*!CLKB*DA*!DB*!Q" \
	{ DCMUX }

define_leakage \
	-pinlist {CLKA CLKB DA DB MA MB Q} \
	-vector {1 1 1 1 1 1 1} \
	-when "CLKA*CLKB*DA*DB*Q" \
	{ DCMUX }

define_leakage \
	-pinlist {CLKA CLKB DA DB MA MB Q} \
	-vector {0 0 0 0 0 0 0} \
	-when "!CLKA*!CLKB*!DA*!DB*!Q" \
	{ DCMUX }

define_leakage \
	-pinlist {CLKA CLKB DA DB MA MB Q} \
	-vector {1 0 0 1 1 1 1} \
	-when "CLKA*!CLKB*!DA*DB*Q" \
	{ DCMUX }

define_leakage \
	-pinlist {CLKA CLKB DA DB MA MB Q} \
	-vector {1 0 1 1 0 1 0} \
	-when "CLKA*!CLKB*DA*DB*!Q" \
	{ DCMUX }

define_leakage \
	-pinlist {CLKA CLKB DA DB MA MB Q} \
	-vector {0 1 0 0 0 1 1} \
	-when "!CLKA*CLKB*!DA*!DB*Q" \
	{ DCMUX }

define_leakage \
	-pinlist {CLKA CLKB DA DB MA MB Q} \
	-vector {0 0 0 1 0 1 0} \
	-when "!CLKA*!CLKB*!DA*DB*!Q" \
	{ DCMUX }

define_leakage \
	-pinlist {CLKA CLKB DA DB MA MB Q} \
	-vector {0 0 1 1 1 1 1} \
	-when "!CLKA*!CLKB*DA*DB*Q" \
	{ DCMUX }

define_leakage \
	-pinlist {CLKA CLKB DA DB MA MB Q} \
	-vector {1 1 0 1 0 1 1} \
	-when "CLKA*CLKB*!DA*DB*Q" \
	{ DCMUX }

define_leakage \
	-pinlist {CLKA CLKB DA DB MA MB Q} \
	-vector {1 0 0 0 1 0 1} \
	-when "CLKA*!CLKB*!DA*!DB*Q" \
	{ DCMUX }

define_leakage \
	-pinlist {CLKA CLKB DA DB MA MB Q} \
	-vector {0 1 1 1 1 0 0} \
	-when "!CLKA*CLKB*DA*DB*!Q" \
	{ DCMUX }

define_leakage \
	-pinlist {CLKA CLKB DA DB MA MB Q} \
	-vector {1 1 0 0 0 1 1} \
	-when "CLKA*CLKB*!DA*!DB*Q" \
	{ DCMUX }

define_leakage \
	-pinlist {CLKA CLKB DA DB MA MB Q} \
	-vector {1 1 0 1 1 1 1} \
	-when "CLKA*CLKB*!DA*DB*Q" \
	{ DCMUX }

define_leakage \
	-pinlist {CLKA CLKB DA DB MA MB Q} \
	-vector {1 1 1 1 0 1 1} \
	-when "CLKA*CLKB*DA*DB*Q" \
	{ DCMUX }

define_leakage \
	-pinlist {CLKA CLKB DA DB MA MB Q} \
	-vector {1 1 1 1 1 0 1} \
	-when "CLKA*CLKB*DA*DB*Q" \
	{ DCMUX }

define_leakage \
	-pinlist {CLKA CLKB DA DB MA MB Q} \
	-vector {0 0 0 0 0 0 1} \
	-when "!CLKA*!CLKB*!DA*!DB*Q" \
	{ DCMUX }

define_leakage \
	-pinlist {CLKA CLKB DA DB MA MB Q} \
	-vector {1 1 1 0 0 0 0} \
	-when "CLKA*CLKB*DA*!DB*!Q" \
	{ DCMUX }

define_leakage \
	-pinlist {CLKA CLKB DA DB MA MB Q} \
	-vector {1 1 0 1 0 0 0} \
	-when "CLKA*CLKB*!DA*DB*!Q" \
	{ DCMUX }

define_leakage \
	-pinlist {CLKA CLKB DA DB MA MB Q} \
	-vector {1 1 0 0 1 0 1} \
	-when "CLKA*CLKB*!DA*!DB*Q" \
	{ DCMUX }

define_leakage \
	-pinlist {CLKA CLKB DA DB MA MB Q} \
	-vector {1 1 1 0 1 1 1} \
	-when "CLKA*CLKB*DA*!DB*Q" \
	{ DCMUX }

define_leakage \
	-pinlist {CLKA CLKB DA DB MA MB Q} \
	-vector {0 0 1 1 1 1 0} \
	-when "!CLKA*!CLKB*DA*DB*!Q" \
	{ DCMUX }

define_leakage \
	-pinlist {CLKA CLKB DA DB MA MB Q} \
	-vector {1 1 1 0 0 1 1} \
	-when "CLKA*CLKB*DA*!DB*Q" \
	{ DCMUX }

define_leakage \
	-pinlist {CLKA CLKB DA DB MA MB Q} \
	-vector {1 1 0 0 1 1 1} \
	-when "CLKA*CLKB*!DA*!DB*Q" \
	{ DCMUX }

define_leakage \
	-pinlist {CLKA CLKB DA DB MA MB Q} \
	-vector {1 1 0 1 1 0 1} \
	-when "CLKA*CLKB*!DA*DB*Q" \
	{ DCMUX }

define_leakage \
	-pinlist {CLKA CLKB DA DB MA MB Q} \
	-vector {1 1 1 1 0 0 0} \
	-when "CLKA*CLKB*DA*DB*!Q" \
	{ DCMUX }

define_arc \
	-type non_seq_setup \
	-pinlist {CLKA CLKB DA DB MA MB Q} \
	-ic "$VDD $VDD $VDD 0 $VDD 0 $VDD" \
	-vector {F F 1 0 X X X} \
	-related_pin CLKA \
	-pin CLKB \
	-probe {Q} \
	{ DCMUX }

define_arc \
	-type non_seq_hold \
	-pinlist {CLKA CLKB DA DB MA MB Q} \
	-ic "$VDD $VDD $VDD 0 $VDD 0 $VDD" \
	-vector {F F 1 0 X X X} \
	-related_pin CLKA \
	-pin CLKB \
	-probe {Q} \
	{ DCMUX }

define_arc \
	-type setup \
	-pinlist {CLKA CLKB DA DB MA MB Q} \
	-ic "0 $VDD $VDD $VDD $VDD $VDD $VDD" \
	-vector {R 1 F 1 X X X} \
	-related_pin CLKA \
	-pin DA \
	-probe {MA} \
	{ DCMUX }

define_arc \
	-type hold \
	-pinlist {CLKA CLKB DA DB MA MB Q} \
	-ic "0 $VDD $VDD $VDD $VDD $VDD $VDD" \
	-vector {R 1 F 1 X X X} \
	-related_pin CLKA \
	-pin DA \
	-probe {MA} \
	{ DCMUX }

define_arc \
	-type setup \
	-pinlist {CLKA CLKB DA DB MA MB Q} \
	-ic "0 $VDD $VDD 0 $VDD 0 0" \
	-vector {R 1 F 0 X X X} \
	-related_pin CLKA \
	-pin DA \
	-probe {Q MA} \
	{ DCMUX }

define_arc \
	-type hold \
	-pinlist {CLKA CLKB DA DB MA MB Q} \
	-ic "0 $VDD $VDD 0 $VDD 0 0" \
	-vector {R 1 F 0 X X X} \
	-related_pin CLKA \
	-pin DA \
	-probe {Q MA} \
	{ DCMUX }

define_arc \
	-type setup \
	-pinlist {CLKA CLKB DA DB MA MB Q} \
	-ic "0 $VDD 0 $VDD 0 $VDD $VDD" \
	-vector {R 1 R 1 X X X} \
	-related_pin CLKA \
	-pin DA \
	-probe {MA} \
	{ DCMUX }

define_arc \
	-type hold \
	-pinlist {CLKA CLKB DA DB MA MB Q} \
	-ic "0 $VDD 0 $VDD 0 $VDD $VDD" \
	-vector {R 1 R 1 X X X} \
	-related_pin CLKA \
	-pin DA \
	-probe {MA} \
	{ DCMUX }

define_arc \
	-type setup \
	-pinlist {CLKA CLKB DA DB MA MB Q} \
	-ic "0 $VDD 0 0 0 0 0" \
	-vector {R 1 R 0 X X X} \
	-related_pin CLKA \
	-pin DA \
	-probe {Q MA} \
	{ DCMUX }

define_arc \
	-type hold \
	-pinlist {CLKA CLKB DA DB MA MB Q} \
	-ic "0 $VDD 0 0 0 0 0" \
	-vector {R 1 R 0 X X X} \
	-related_pin CLKA \
	-pin DA \
	-probe {Q MA} \
	{ DCMUX }

define_arc \
	-type setup \
	-pinlist {CLKA CLKB DA DB MA MB Q} \
	-ic "$VDD 0 $VDD $VDD $VDD $VDD $VDD" \
	-vector {1 R 1 F X X X} \
	-related_pin CLKB \
	-pin DB \
	-probe {MB} \
	{ DCMUX }

define_arc \
	-type hold \
	-pinlist {CLKA CLKB DA DB MA MB Q} \
	-ic "$VDD 0 $VDD $VDD $VDD $VDD $VDD" \
	-vector {1 R 1 F X X X} \
	-related_pin CLKB \
	-pin DB \
	-probe {MB} \
	{ DCMUX }

define_arc \
	-type setup \
	-pinlist {CLKA CLKB DA DB MA MB Q} \
	-ic "$VDD 0 0 $VDD 0 $VDD 0" \
	-vector {1 R 0 F X X X} \
	-related_pin CLKB \
	-pin DB \
	-probe {Q MB} \
	{ DCMUX }

define_arc \
	-type hold \
	-pinlist {CLKA CLKB DA DB MA MB Q} \
	-ic "$VDD 0 0 $VDD 0 $VDD 0" \
	-vector {1 R 0 F X X X} \
	-related_pin CLKB \
	-pin DB \
	-probe {Q MB} \
	{ DCMUX }

define_arc \
	-type setup \
	-pinlist {CLKA CLKB DA DB MA MB Q} \
	-ic "$VDD 0 $VDD 0 $VDD 0 $VDD" \
	-vector {1 R 1 R X X X} \
	-related_pin CLKB \
	-pin DB \
	-probe {MB} \
	{ DCMUX }

define_arc \
	-type hold \
	-pinlist {CLKA CLKB DA DB MA MB Q} \
	-ic "$VDD 0 $VDD 0 $VDD 0 $VDD" \
	-vector {1 R 1 R X X X} \
	-related_pin CLKB \
	-pin DB \
	-probe {MB} \
	{ DCMUX }

define_arc \
	-type setup \
	-pinlist {CLKA CLKB DA DB MA MB Q} \
	-ic "$VDD 0 0 0 0 0 0" \
	-vector {1 R 0 R X X X} \
	-related_pin CLKB \
	-pin DB \
	-probe {Q MB} \
	{ DCMUX }

define_arc \
	-type hold \
	-pinlist {CLKA CLKB DA DB MA MB Q} \
	-ic "$VDD 0 0 0 0 0 0" \
	-vector {1 R 0 R X X X} \
	-related_pin CLKB \
	-pin DB \
	-probe {Q MB} \
	{ DCMUX }

define_arc \
	-type combinational \
	-pinlist {CLK EN GCLK} \
	-ic "0 $VDD 0" \
	-vector {R 1 R} \
	-related_pin CLK \
	-pin GCLK \
	{ ICG }

define_arc \
	-type combinational \
	-pinlist {CLK EN GCLK} \
	-ic "$VDD $VDD $VDD" \
	-vector {F 1 F} \
	-related_pin CLK \
	-pin GCLK \
	{ ICG }

define_arc \
	-type hidden \
	-pinlist {CLK EN GCLK} \
	-ic "0 0 0" \
	-vector {R 0 0} \
	-pin CLK \
	{ ICG }

define_arc \
	-type hidden \
	-pinlist {CLK EN GCLK} \
	-ic "0 0 0" \
	-vector {0 R 0} \
	-pin EN \
	{ ICG }

define_arc \
	-type hidden \
	-pinlist {CLK EN GCLK} \
	-ic "0 $VDD 0" \
	-vector {0 F 0} \
	-pin EN \
	{ ICG }

define_arc \
	-type hidden \
	-pinlist {CLK EN GCLK} \
	-ic "$VDD 0 0" \
	-vector {F 0 0} \
	-pin CLK \
	{ ICG }

define_leakage -when "!CLK*!EN*!GCLK" { ICG }

define_leakage -when "!CLK*EN*!GCLK" { ICG }

define_leakage \
	-pinlist {CLK EN GCLK} \
	-vector {1 0 0} \
	-when "CLK*!EN*!GCLK" \
	{ ICG }

define_leakage \
	-pinlist {CLK EN GCLK} \
	-vector {1 1 1} \
	-when "CLK*EN*GCLK" \
	{ ICG }

define_leakage \
	-pinlist {CLK EN GCLK} \
	-vector {1 0 1} \
	-when "CLK*!EN*GCLK" \
	{ ICG }

define_leakage \
	-pinlist {CLK EN GCLK} \
	-vector {1 1 0} \
	-when "CLK*EN*!GCLK" \
	{ ICG }

define_arc \
	-type setup \
	-pinlist {CLK EN EL GCLK} \
	-ic "0 $VDD $VDD 0" \
	-vector {R F X X} \
	-related_pin CLK \
	-pin EN \
	-probe {EL} \
	{ ICG }

define_arc \
	-type hold \
	-pinlist {CLK EN EL GCLK} \
	-ic "0 $VDD $VDD 0" \
	-vector {R F X X} \
	-related_pin CLK \
	-pin EN \
	-probe {EL} \
	{ ICG }

define_arc \
	-type setup \
	-pinlist {CLK EN EL GCLK} \
	-ic "0 0 0 0" \
	-vector {R R X X} \
	-related_pin CLK \
	-pin EN \
	-probe {EL} \
	{ ICG }

define_arc \
	-type hold \
	-pinlist {CLK EN EL GCLK} \
	-ic "0 0 0 0" \
	-vector {R R X X} \
	-related_pin CLK \
	-pin EN \
	-probe {EL} \
	{ ICG }

define_arc \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-ic "0 0 0 $VDD 0 $VDD $VDD $VDD 0 0 0 0" \
	-vector {R 0 0 1 0 X X X X X X R} \
	-related_pin CLKA \
	-pin GCLK \
	{ ICM }

define_arc \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-ic "0 0 $VDD 0 $VDD 0 0 0 $VDD $VDD $VDD 0" \
	-vector {0 R 1 0 1 X X X X X X R} \
	-related_pin CLKB \
	-pin GCLK \
	{ ICM }

define_arc \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-ic "$VDD $VDD $VDD 0 $VDD 0 0 0 $VDD $VDD $VDD $VDD" \
	-vector {1 F 1 0 1 X X X X X X F} \
	-related_pin CLKB \
	-pin GCLK \
	{ ICM }

define_arc \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-ic "$VDD $VDD $VDD 0 $VDD 0 0 0 $VDD $VDD $VDD $VDD" \
	-vector {1 1 1 R 1 X X X X X X F} \
	-related_pin RB \
	-pin GCLK \
	{ ICM }

define_arc \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-ic "$VDD $VDD 0 $VDD 0 $VDD $VDD $VDD 0 0 0 $VDD" \
	-vector {F 1 0 1 0 X X X X X X F} \
	-related_pin CLKA \
	-pin GCLK \
	{ ICM }

define_arc \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-ic "$VDD $VDD 0 $VDD 0 $VDD $VDD $VDD 0 0 0 $VDD" \
	-vector {1 1 R 1 0 X X X X X X F} \
	-related_pin RA \
	-pin GCLK \
	{ ICM }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-ic "0 0 $VDD $VDD 0 0 0 0 0 0 0 0" \
	-vector {R 0 1 1 0 X X X X X X 0} \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-ic "0 0 $VDD $VDD 0 0 0 0 0 0 0 0" \
	-vector {0 R 1 1 0 X X X X X X 0} \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-ic "0 0 $VDD $VDD 0 0 0 0 0 0 0 0" \
	-vector {0 0 F 1 0 X X X X X X 0} \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-ic "0 0 $VDD $VDD 0 0 0 0 0 0 0 0" \
	-vector {0 0 1 F 0 X X X X X X 0} \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-ic "0 0 $VDD $VDD 0 0 0 0 0 0 0 0" \
	-vector {0 0 1 1 R X X X X X X 0} \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-ic "0 0 $VDD $VDD $VDD 0 0 0 0 0 0 0" \
	-vector {0 0 1 1 F X X X X X X 0} \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-ic "0 $VDD $VDD $VDD 0 0 0 0 0 0 0 0" \
	-vector {0 F 1 1 0 X X X X X X 0} \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-ic "$VDD 0 $VDD $VDD 0 0 0 0 0 0 0 0" \
	-vector {F 0 1 1 0 X X X X X X 0} \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-ic "0 0 0 $VDD $VDD 0 0 0 0 0 0 0" \
	-vector {0 0 R 1 1 X X X X X X 0} \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-ic "$VDD 0 $VDD 0 0 0 0 0 0 0 0 0" \
	-vector {1 0 1 R 0 X X X X X X 0} \
	-pin RB \
	{ ICM }

define_leakage -when "!CLKA*!CLKB*!GCLK*RA*RB*!S" { ICM }

define_leakage -when "!CLKA*!CLKB*!GCLK*RA*RB*S" { ICM }

define_leakage -when "!CLKA*CLKB*!GCLK*RA*RB*!S" { ICM }

define_leakage -when "!CLKA*CLKB*!GCLK*RA*RB*S" { ICM }

define_leakage -when "CLKA*!CLKB*!GCLK*RA*RB*!S" { ICM }

define_leakage -when "CLKA*!CLKB*!GCLK*RA*RB*S" { ICM }

define_leakage -when "CLKA*CLKB*!GCLK*RA*RB*!S" { ICM }

define_leakage -when "CLKA*CLKB*!GCLK*RA*RB*S" { ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 0 0 1 1 0 0 0 0 0 0 0} \
	-when "!CLKA*!CLKB*!GCLK*!RA*RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 0 0 1 1 0 0 0 0 0 0 0} \
	-when "CLKA*!CLKB*!GCLK*!RA*RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 0 1 0 0 0 0 0 0 0 0 0} \
	-when "CLKA*!CLKB*!GCLK*RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 0 1 0 1 0 0 0 1 0 0 0} \
	-when "CLKA*!CLKB*!GCLK*RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 0 1 0 0 0 0 0 0 0 0 0} \
	-when "!CLKA*!CLKB*!GCLK*RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 1 0 0 0 0 0 0 0 0 0} \
	-when "CLKA*CLKB*!GCLK*RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 1 0 1 0 0 0 0 0 0 0} \
	-when "CLKA*CLKB*!GCLK*RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 1 1 0 1 0 0 0 0 0 0 0} \
	-when "!CLKA*CLKB*!GCLK*RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 1 0 1 1 0 0 0 0 0 0 0} \
	-when "!CLKA*CLKB*!GCLK*!RA*RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 0 1 1 0 0 0 0 0 0 0} \
	-when "CLKA*CLKB*!GCLK*!RA*RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 0 0 1 0 1 0 0 0 0 0 0} \
	-when "!CLKA*!CLKB*!GCLK*!RA*RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 1 0 1 0 1 0 0 0 0 0 0} \
	-when "!CLKA*CLKB*!GCLK*!RA*RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 0 0 1 0 0 0 0 0 0 0 0} \
	-when "CLKA*!CLKB*!GCLK*!RA*RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 1 1 0 0 0 0 0 0 0 0 0} \
	-when "!CLKA*CLKB*!GCLK*RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 0 1 0 0 0 0 0 0 0 0} \
	-when "CLKA*CLKB*!GCLK*!RA*RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 0 1 0 1 0 0 0 1 0 0 0} \
	-when "!CLKA*!CLKB*!GCLK*RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 1 0 0 0 1 0 0 0 0 0 0} \
	-when "!CLKA*CLKB*!GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 0 0 1 0 0 0 0 0 0 0} \
	-when "CLKA*CLKB*!GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 1 0 0 1 0 0 0 0 0 0 0} \
	-when "!CLKA*CLKB*!GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 0 0 1 0 1 1 1 0 0 0 0} \
	-when "!CLKA*!CLKB*!GCLK*!RA*RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 1 0 1 0 1 1 1 0 0 0 0} \
	-when "!CLKA*CLKB*!GCLK*!RA*RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 0 0 0 1 0 0 0 1 0 0 0} \
	-when "!CLKA*!CLKB*!GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 1 0 1 0 0 0 1 1 0 0} \
	-when "CLKA*CLKB*!GCLK*RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 0 0 0 1 0 0 0 1 0 0 0} \
	-when "CLKA*!CLKB*!GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 0 0 0 0 0 0 0 0 0 0} \
	-when "CLKA*CLKB*!GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 0 0 0 0 1 0 0 0 0 0 0} \
	-when "!CLKA*!CLKB*!GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 1 1 0 1 0 0 0 1 1 0 0} \
	-when "!CLKA*CLKB*!GCLK*RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 0 1 0 1 0 0 0 1 1 1 0} \
	-when "!CLKA*!CLKB*!GCLK*RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 0 1 0 1 0 0 0 1 1 1 0} \
	-when "CLKA*!CLKB*!GCLK*RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 0 0 1 0 1 1 0 0 0 0 0} \
	-when "CLKA*!CLKB*!GCLK*!RA*RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 0 0 0 0 0 0 0 0 0 0 0} \
	-when "CLKA*!CLKB*!GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 0 1 0 1 1 0 0 0 0 0} \
	-when "CLKA*CLKB*!GCLK*!RA*RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 1 0 1 1 0 1 1 0 0 0 0} \
	-when "!CLKA*CLKB*!GCLK*!RA*RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 0 0 0 1 1 0 0 0 0 0} \
	-when "CLKA*CLKB*!GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 0 0 0 0 1 1 1 0 0 0 0} \
	-when "!CLKA*!CLKB*!GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 0 0 1 1 1 1 0 0 0 0 0} \
	-when "CLKA*!CLKB*!GCLK*!RA*RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 0 0 0 1 0 0 0 1 1 1 0} \
	-when "!CLKA*!CLKB*!GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 0 0 0 1 0 0 0 1 1 1 0} \
	-when "CLKA*!CLKB*!GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 1 0 1 0 0 0 1 1 1 1} \
	-when "CLKA*CLKB*GCLK*RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 0 0 0 0 1 1 0 0 0 0 0} \
	-when "CLKA*!CLKB*!GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 0 1 0 0 0 0 0 0 1 1 0} \
	-when "!CLKA*!CLKB*!GCLK*RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 0 1 0 1 1 1 0 0 0 1} \
	-when "CLKA*CLKB*GCLK*!RA*RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 1 0 0 0 0 0 1 1 0 0} \
	-when "CLKA*CLKB*!GCLK*RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 0 0 1 0 0 0 1 1 0 0} \
	-when "CLKA*CLKB*!GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 1 1 0 0 0 0 0 1 1 0 0} \
	-when "!CLKA*CLKB*!GCLK*RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 1 0 0 0 1 1 1 0 0 0 0} \
	-when "!CLKA*CLKB*!GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 0 0 1 1 0 1 1 0 0 0 0} \
	-when "!CLKA*!CLKB*!GCLK*!RA*RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 0 0 1 0 1 1 1 0 0 0 1} \
	-when "CLKA*!CLKB*GCLK*!RA*RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 0 1 0 0 0 0 0 0 1 1 0} \
	-when "CLKA*!CLKB*!GCLK*RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 1 0 0 1 0 0 0 1 1 0 0} \
	-when "!CLKA*CLKB*!GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 0 1 1 1 1 0 0 0 0 0} \
	-when "CLKA*CLKB*!GCLK*!RA*RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 1 1 0 1 0 0 0 1 1 1 1} \
	-when "!CLKA*CLKB*GCLK*RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 1 0 0 0 1 0 0 1 1 0 0} \
	-when "!CLKA*CLKB*!GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 0 0 1 0 0 0 1 1 1 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 0 0 0 1 1 1 0 1 0 0 0} \
	-when "CLKA*!CLKB*!GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 1 0 0 0 0 0 1 1 1 1} \
	-when "CLKA*CLKB*GCLK*RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 0 0 0 0 0 0 0 0 1 1 0} \
	-when "!CLKA*!CLKB*!GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 0 0 0 0 0 0 1 1 0 0} \
	-when "CLKA*CLKB*!GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 1 0 0 1 0 0 0 1 1 1 1} \
	-when "!CLKA*CLKB*GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 0 0 1 1 1 0 0 0 0 0} \
	-when "CLKA*CLKB*!GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 0 0 0 1 1 1 0 0 0 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 0 0 0 0 1 1 1 0 0 0 1} \
	-when "CLKA*!CLKB*GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 0 0 0 1 0 1 1 0 0 0 0} \
	-when "!CLKA*!CLKB*!GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 0 0 1 1 1 1 1 0 0 0 1} \
	-when "CLKA*!CLKB*GCLK*!RA*RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 1 0 0 1 0 1 1 0 0 0 0} \
	-when "!CLKA*CLKB*!GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 0 0 1 1 0 0 1 0 0 0 1} \
	-when "CLKA*!CLKB*GCLK*!RA*RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 0 0 0 0 0 0 0 0 1 1 0} \
	-when "CLKA*!CLKB*!GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 1 0 0 0 0 0 0 0 1 1} \
	-when "CLKA*CLKB*GCLK*RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 0 1 1 0 0 1 0 0 0 1} \
	-when "CLKA*CLKB*GCLK*!RA*RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 0 1 1 1 1 1 0 0 0 1} \
	-when "CLKA*CLKB*GCLK*!RA*RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 1 1 0 0 0 0 0 1 1 1 1} \
	-when "!CLKA*CLKB*GCLK*RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 1 1 0 0 0 0 0 0 0 1 1} \
	-when "!CLKA*CLKB*GCLK*RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 0 0 1 1 1 1 0 0 0 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 0 0 0 1 0 0 1 0 0 0 1} \
	-when "CLKA*!CLKB*GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 0 1 0 0 0 1 0 0 0 1} \
	-when "CLKA*CLKB*GCLK*!RA*RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 0 0 1 1 1 0 1 1 0 0} \
	-when "CLKA*CLKB*!GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 0 0 1 0 0 0 1 0 0 0 1} \
	-when "CLKA*!CLKB*GCLK*!RA*RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 1 1 0 1 0 0 0 0 0 1 1} \
	-when "!CLKA*CLKB*GCLK*RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 0 0 0 0 0 0 0 0 1 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 0 0 0 1 1 0 1 1 0 0} \
	-when "CLKA*CLKB*!GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 0 0 0 1 1 1 1 0 0 0 1} \
	-when "CLKA*!CLKB*GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 1 0 0 0 0 0 0 0 0 1 1} \
	-when "!CLKA*CLKB*GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 0 0 1 0 0 1 0 0 0 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 0 0 0 0 0 0 1 1 1 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 1 0 1 0 0 0 0 0 1 1} \
	-when "CLKA*CLKB*GCLK*RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 1 0 0 0 0 0 0 1 1 1 1} \
	-when "!CLKA*CLKB*GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 0 0 0 1 1 1 0 1 1 1 0} \
	-when "CLKA*!CLKB*!GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 1 0 0 1 0 0 0 0 0 1 1} \
	-when "!CLKA*CLKB*GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 0 0 0 0 0 1 0 0 0 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 0 0 0 0 1 1 0 0 1 1 0} \
	-when "CLKA*!CLKB*!GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 1 0 0 1 0 1 1 1 1 0 0} \
	-when "!CLKA*CLKB*!GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 0 0 1 0 0 0 0 0 1 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 1 0 0 0 1 1 1 1 1 0 0} \
	-when "!CLKA*CLKB*!GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 0 0 0 0 0 0 1 0 0 0 1} \
	-when "CLKA*!CLKB*GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 0 0 0 1 0 1 1 0 1 1 0} \
	-when "!CLKA*!CLKB*!GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 0 0 0 1 1 1 1 1 0 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 0 0 1 0 0 1 1 1 0 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 0 0 0 1 1 0 0 0 1 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 0 0 1 1 1 0 1 1 1 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 0 0 0 0 0 1 1 0 1 1 0} \
	-when "!CLKA*!CLKB*!GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 0 0 0 0 0 0 1 0 1 1 1} \
	-when "CLKA*!CLKB*GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 0 0 0 0 0 1 1 1 0 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 0 0 0 0 1 1 1 0 1 1 1} \
	-when "CLKA*!CLKB*GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 0 0 1 1 1 0 0 0 1 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 1 0 0 0 0 1 1 0 0 1 1} \
	-when "!CLKA*CLKB*GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 0 0 0 1 0 0 1 0 1 1 1} \
	-when "CLKA*!CLKB*GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 0 0 1 1 1 1 1 1 0 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 0 0 0 1 1 0 1 1 1 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 1 0 0 1 0 1 1 0 0 1 1} \
	-when "!CLKA*CLKB*GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 1 0 0 1 0 1 1 1 1 1 1} \
	-when "!CLKA*CLKB*GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 0 0 1 0 0 1 1 1 1 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 0 0 0 1 1 1 1 0 1 1 1} \
	-when "CLKA*!CLKB*GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 0 0 1 0 0 1 0 0 1 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 1 0 0 0 0 1 1 1 1 1 1} \
	-when "!CLKA*CLKB*GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 0 0 0 0 0 1 0 0 1 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 0 0 0 1 1 1 0 0 1 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 0 0 0 0 0 1 1 1 1 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 0 0 1 1 1 1 0 0 1 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*S" \
	{ ICM }

define_arc \
	-type setup \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-ic "0 0 $VDD $VDD 0 0 0 0 0 0 0 0" \
	-vector {R 0 F 1 0 X X X X X X X} \
	-related_pin CLKA \
	-pin RA \
	-probe {sela1 sela2} \
	{ ICM }

define_arc \
	-type hold \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-ic "0 0 $VDD $VDD 0 0 0 0 0 0 0 0" \
	-vector {R 0 F 1 0 X X X X X X X} \
	-related_pin CLKA \
	-pin RA \
	-probe {sela1 sela2} \
	{ ICM }

define_arc \
	-type setup \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-ic "0 0 0 $VDD $VDD 0 0 0 0 0 0 0" \
	-vector {R 0 0 1 F X X X X X X X} \
	-related_pin CLKA \
	-pin S \
	-probe {sela1 sela2} \
	{ ICM }

define_arc \
	-type hold \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-ic "0 0 0 $VDD $VDD 0 0 0 0 0 0 0" \
	-vector {R 0 0 1 F X X X X X X X} \
	-related_pin CLKA \
	-pin S \
	-probe {sela1 sela2} \
	{ ICM }

define_arc \
	-type setup \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-ic "0 0 0 $VDD 0 $VDD 0 0 0 0 0 0" \
	-vector {R 0 0 1 R X X X X X X X} \
	-related_pin CLKA \
	-pin S \
	-probe {sela1 sela2} \
	{ ICM }

define_arc \
	-type hold \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-ic "0 0 0 $VDD 0 $VDD 0 0 0 0 0 0" \
	-vector {R 0 0 1 R X X X X X X X} \
	-related_pin CLKA \
	-pin S \
	-probe {sela1 sela2} \
	{ ICM }

define_arc \
	-type setup \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-ic "0 0 $VDD $VDD $VDD 0 0 0 0 0 0 0" \
	-vector {0 R 1 F 1 X X X X X X X} \
	-related_pin CLKB \
	-pin RB \
	-probe {selb1 selb2} \
	{ ICM }

define_arc \
	-type hold \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-ic "0 0 $VDD $VDD $VDD 0 0 0 0 0 0 0" \
	-vector {0 R 1 F 1 X X X X X X X} \
	-related_pin CLKB \
	-pin RB \
	-probe {selb1 selb2} \
	{ ICM }

define_arc \
	-type setup \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-ic "$VDD 0 $VDD 0 $VDD 0 0 0 $VDD 0 0 0" \
	-vector {1 R 1 0 F X X X X X X X} \
	-related_pin CLKB \
	-pin S \
	-probe {selb1 selb2} \
	{ ICM }

define_arc \
	-type hold \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-ic "$VDD 0 $VDD 0 $VDD 0 0 0 $VDD 0 0 0" \
	-vector {1 R 1 0 F X X X X X X X} \
	-related_pin CLKB \
	-pin S \
	-probe {selb1 selb2} \
	{ ICM }

define_arc \
	-type setup \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-ic "$VDD 0 $VDD 0 0 0 0 0 0 0 0 0" \
	-vector {1 R 1 0 R X X X X X X X} \
	-related_pin CLKB \
	-pin S \
	-probe {selb1 selb2} \
	{ ICM }

define_arc \
	-type hold \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-ic "$VDD 0 $VDD 0 0 0 0 0 0 0 0 0" \
	-vector {1 R 1 0 R X X X X X X X} \
	-related_pin CLKB \
	-pin S \
	-probe {selb1 selb2} \
	{ ICM }

define_arc \
	-type combinational \
	-pinlist {C D Y} \
	-ic "0 $VDD 0" \
	-vector {R 1 R} \
	-related_pin C \
	-pin Y \
	{ GL }

define_arc \
	-type combinational \
	-pinlist {C D Y} \
	-ic "$VDD $VDD $VDD" \
	-vector {F 1 F} \
	-related_pin C \
	-pin Y \
	{ GL }

define_arc \
	-type hidden \
	-pinlist {C D Y} \
	-ic "0 0 0" \
	-vector {R 0 0} \
	-pin C \
	{ GL }

define_arc \
	-type hidden \
	-pinlist {C D Y} \
	-ic "0 0 0" \
	-vector {0 R 0} \
	-pin D \
	{ GL }

define_arc \
	-type hidden \
	-pinlist {C D Y} \
	-ic "0 $VDD 0" \
	-vector {0 F 0} \
	-pin D \
	{ GL }

define_arc \
	-type hidden \
	-pinlist {C D Y} \
	-ic "$VDD 0 0" \
	-vector {F 0 0} \
	-pin C \
	{ GL }

define_leakage -when "!C*!D*!Y" { GL }

define_leakage -when "!C*D*!Y" { GL }

define_leakage \
	-pinlist {C D Y} \
	-vector {1 0 0} \
	-when "C*!D*!Y" \
	{ GL }

define_leakage \
	-pinlist {C D Y} \
	-vector {1 1 1} \
	-when "C*D*Y" \
	{ GL }

define_leakage \
	-pinlist {C D Y} \
	-vector {1 1 0} \
	-when "C*D*!Y" \
	{ GL }

define_leakage \
	-pinlist {C D Y} \
	-vector {1 0 1} \
	-when "C*!D*Y" \
	{ GL }

define_arc \
	-type non_seq_setup \
	-pinlist {C D L Y} \
	-ic "0 $VDD $VDD 0" \
	-vector {R F X X} \
	-related_pin C \
	-pin D \
	-probe {L} \
	{ GL }

define_arc \
	-type non_seq_hold \
	-pinlist {C D L Y} \
	-ic "0 $VDD $VDD 0" \
	-vector {R F X X} \
	-related_pin C \
	-pin D \
	-probe {L} \
	{ GL }

define_arc \
	-type non_seq_setup \
	-pinlist {C D L Y} \
	-ic "0 0 0 0" \
	-vector {R R X X} \
	-related_pin C \
	-pin D \
	-probe {L} \
	{ GL }

define_arc \
	-type non_seq_hold \
	-pinlist {C D L Y} \
	-ic "0 0 0 0" \
	-vector {R R X X} \
	-related_pin C \
	-pin D \
	-probe {L} \
	{ GL }

define_arc \
	-type combinational \
	-pinlist {A B C D Y Z} \
	-ic "0 0 0 $VDD 0 0" \
	-vector {0 0 R 1 R X} \
	-related_pin C \
	-pin Y \
	{ MIX }

define_arc \
	-type combinational \
	-pinlist {A B C D Y Z} \
	-ic "0 $VDD 0 0 0 0" \
	-vector {R 1 0 0 X R} \
	-related_pin A \
	-pin Z \
	{ MIX }

define_arc \
	-type combinational \
	-pinlist {A B C D Y Z} \
	-ic "$VDD 0 0 0 0 0" \
	-vector {1 R 0 0 X R} \
	-related_pin B \
	-pin Z \
	{ MIX }

define_arc \
	-type combinational \
	-pinlist {A B C D Y Z} \
	-ic "$VDD $VDD 0 0 0 $VDD" \
	-vector {F 1 0 0 X F} \
	-related_pin A \
	-pin Z \
	{ MIX }

define_arc \
	-type combinational \
	-pinlist {A B C D Y Z} \
	-ic "$VDD $VDD 0 0 0 $VDD" \
	-vector {1 F 0 0 X F} \
	-related_pin B \
	-pin Z \
	{ MIX }

define_arc \
	-type combinational \
	-pinlist {A B C D Y Z} \
	-ic "$VDD $VDD $VDD $VDD $VDD $VDD" \
	-vector {1 1 F 1 F X} \
	-related_pin C \
	-pin Y \
	{ MIX }

define_arc \
	-type hidden \
	-pinlist {A B C D Y Z} \
	-ic "0 0 0 0 0 0" \
	-vector {R 0 0 0 0 0} \
	-pin A \
	{ MIX }

define_arc \
	-type hidden \
	-pinlist {A B C D Y Z} \
	-ic "0 0 0 0 0 0" \
	-vector {0 R 0 0 0 0} \
	-pin B \
	{ MIX }

define_arc \
	-type hidden \
	-pinlist {A B C D Y Z} \
	-ic "0 0 0 0 0 0" \
	-vector {0 0 R 0 0 0} \
	-pin C \
	{ MIX }

define_arc \
	-type hidden \
	-pinlist {A B C D Y Z} \
	-ic "0 0 0 0 0 0" \
	-vector {0 0 0 R 0 0} \
	-pin D \
	{ MIX }

define_arc \
	-type hidden \
	-pinlist {A B C D Y Z} \
	-ic "0 0 0 $VDD 0 0" \
	-vector {0 0 0 F 0 0} \
	-pin D \
	{ MIX }

define_arc \
	-type hidden \
	-pinlist {A B C D Y Z} \
	-ic "0 $VDD 0 0 0 0" \
	-vector {0 F 0 0 0 0} \
	-pin B \
	{ MIX }

define_arc \
	-type hidden \
	-pinlist {A B C D Y Z} \
	-ic "$VDD 0 0 0 0 0" \
	-vector {F 0 0 0 0 0} \
	-pin A \
	{ MIX }

define_arc \
	-type hidden \
	-pinlist {A B C D Y Z} \
	-ic "$VDD $VDD $VDD 0 0 $VDD" \
	-vector {1 1 F 0 0 1} \
	-pin C \
	{ MIX }

define_leakage -when "!A*!B*!C*!D*!Y*!Z" { MIX }

define_leakage -when "!A*!B*!C*D*!Y*!Z" { MIX }

define_leakage -when "!A*B*!C*!D*!Y*!Z" { MIX }

define_leakage -when "!A*B*!C*D*!Y*!Z" { MIX }

define_leakage -when "A*!B*!C*!D*!Y*!Z" { MIX }

define_leakage -when "A*!B*!C*D*!Y*!Z" { MIX }

define_leakage -when "A*B*!C*!D*!Y*Z" { MIX }

define_leakage -when "A*B*!C*D*!Y*Z" { MIX }

define_leakage \
	-pinlist {A B C D Y Z} \
	-vector {1 1 1 1 1 1} \
	-when "A*B*C*D*Y*Z" \
	{ MIX }

define_leakage \
	-pinlist {A B C D Y Z} \
	-vector {1 0 1 1 1 0} \
	-when "A*!B*C*D*Y*!Z" \
	{ MIX }

define_leakage \
	-pinlist {A B C D Y Z} \
	-vector {1 1 1 0 0 1} \
	-when "A*B*C*!D*!Y*Z" \
	{ MIX }

define_leakage \
	-pinlist {A B C D Y Z} \
	-vector {0 0 1 0 0 0} \
	-when "!A*!B*C*!D*!Y*!Z" \
	{ MIX }

define_leakage \
	-pinlist {A B C D Y Z} \
	-vector {0 1 1 1 1 0} \
	-when "!A*B*C*D*Y*!Z" \
	{ MIX }

define_leakage \
	-pinlist {A B C D Y Z} \
	-vector {0 0 1 1 1 0} \
	-when "!A*!B*C*D*Y*!Z" \
	{ MIX }

define_leakage \
	-pinlist {A B C D Y Z} \
	-vector {0 1 1 0 0 0} \
	-when "!A*B*C*!D*!Y*!Z" \
	{ MIX }

define_leakage \
	-pinlist {A B C D Y Z} \
	-vector {1 0 1 0 0 0} \
	-when "A*!B*C*!D*!Y*!Z" \
	{ MIX }

define_leakage \
	-pinlist {A B C D Y Z} \
	-vector {0 1 1 0 1 0} \
	-when "!A*B*C*!D*Y*!Z" \
	{ MIX }

define_leakage \
	-pinlist {A B C D Y Z} \
	-vector {1 0 1 1 0 0} \
	-when "A*!B*C*D*!Y*!Z" \
	{ MIX }

define_leakage \
	-pinlist {A B C D Y Z} \
	-vector {0 1 1 1 0 0} \
	-when "!A*B*C*D*!Y*!Z" \
	{ MIX }

define_leakage \
	-pinlist {A B C D Y Z} \
	-vector {1 1 1 1 0 1} \
	-when "A*B*C*D*!Y*Z" \
	{ MIX }

define_leakage \
	-pinlist {A B C D Y Z} \
	-vector {1 0 1 0 1 0} \
	-when "A*!B*C*!D*Y*!Z" \
	{ MIX }

define_leakage \
	-pinlist {A B C D Y Z} \
	-vector {0 0 1 0 1 0} \
	-when "!A*!B*C*!D*Y*!Z" \
	{ MIX }

define_leakage \
	-pinlist {A B C D Y Z} \
	-vector {0 0 1 1 0 0} \
	-when "!A*!B*C*D*!Y*!Z" \
	{ MIX }

define_leakage \
	-pinlist {A B C D Y Z} \
	-vector {1 1 1 0 1 1} \
	-when "A*B*C*!D*Y*Z" \
	{ MIX }

define_arc \
	-type non_seq_setup \
	-pinlist {A B C D L Y Z} \
	-ic "0 0 0 $VDD $VDD 0 0" \
	-vector {0 0 R F X X X} \
	-related_pin C \
	-pin D \
	-probe {L} \
	{ MIX }

define_arc \
	-type non_seq_hold \
	-pinlist {A B C D L Y Z} \
	-ic "0 0 0 $VDD $VDD 0 0" \
	-vector {0 0 R F X X X} \
	-related_pin C \
	-pin D \
	-probe {L} \
	{ MIX }

define_arc \
	-type non_seq_setup \
	-pinlist {A B C D L Y Z} \
	-ic "0 0 0 0 0 0 0" \
	-vector {0 0 R R X X X} \
	-related_pin C \
	-pin D \
	-probe {L} \
	{ MIX }

define_arc \
	-type non_seq_hold \
	-pinlist {A B C D L Y Z} \
	-ic "0 0 0 0 0 0 0" \
	-vector {0 0 R R X X X} \
	-related_pin C \
	-pin D \
	-probe {L} \
	{ MIX }

define_arc \
	-type combinational \
	-pinlist {C D E L Z2} \
	-ic "0 0 0 0 0" \
	-vector {0 0 R X R} \
	-related_pin E \
	-pin Z2 \
	{ TRW }

define_arc \
	-type combinational \
	-pinlist {C D E L Z2} \
	-ic "0 0 $VDD 0 $VDD" \
	-vector {0 0 F X F} \
	-related_pin E \
	-pin Z2 \
	{ TRW }

define_arc \
	-type combinational \
	-pinlist {C D E L Z2} \
	-ic "0 $VDD 0 $VDD 0" \
	-vector {R 1 0 X R} \
	-related_pin C \
	-pin Z2 \
	{ TRW }

define_arc \
	-type combinational \
	-pinlist {C D E L Z2} \
	-ic "$VDD $VDD 0 $VDD $VDD" \
	-vector {F 1 0 X F} \
	-related_pin C \
	-pin Z2 \
	{ TRW }

define_arc \
	-type hidden \
	-pinlist {C D E L Z2} \
	-ic "0 0 0 0 0" \
	-vector {R 0 0 X 0} \
	-pin C \
	{ TRW }

define_arc \
	-type hidden \
	-pinlist {C D E L Z2} \
	-ic "0 0 0 0 0" \
	-vector {0 R 0 X 0} \
	-pin D \
	{ TRW }

define_arc \
	-type hidden \
	-pinlist {C D E L Z2} \
	-ic "0 $VDD 0 $VDD 0" \
	-vector {0 F 0 X 0} \
	-pin D \
	{ TRW }

define_arc \
	-type hidden \
	-pinlist {C D E L Z2} \
	-ic "$VDD $VDD $VDD $VDD $VDD" \
	-vector {F 1 1 X 1} \
	-pin C \
	{ TRW }

define_arc \
	-type hidden \
	-pinlist {C D E L Z2} \
	-ic "$VDD $VDD $VDD $VDD $VDD" \
	-vector {1 1 F X 1} \
	-pin E \
	{ TRW }

define_arc \
	-type hidden \
	-pinlist {C D E L Z2} \
	-ic "$VDD $VDD 0 $VDD $VDD" \
	-vector {1 1 R X 1} \
	-pin E \
	{ TRW }

define_leakage -when "!C*!D*!E*!Z2" { TRW }

define_leakage -when "!C*!D*E*Z2" { TRW }

define_leakage -when "!C*D*!E*!Z2" { TRW }

define_leakage -when "!C*D*E*Z2" { TRW }

define_leakage \
	-pinlist {C D E L Z2} \
	-vector {1 1 1 1 1} \
	-when "C*D*E*Z2" \
	{ TRW }

define_leakage \
	-pinlist {C D E L Z2} \
	-vector {1 0 0 0 0} \
	-when "C*!D*!E*!Z2" \
	{ TRW }

define_leakage \
	-pinlist {C D E L Z2} \
	-vector {1 0 1 0 1} \
	-when "C*!D*E*Z2" \
	{ TRW }

define_leakage \
	-pinlist {C D E L Z2} \
	-vector {1 1 0 1 1} \
	-when "C*D*!E*Z2" \
	{ TRW }

define_leakage \
	-pinlist {C D E L Z2} \
	-vector {1 0 1 1 1} \
	-when "C*!D*E*Z2" \
	{ TRW }

define_leakage \
	-pinlist {C D E L Z2} \
	-vector {1 0 0 1 1} \
	-when "C*!D*!E*Z2" \
	{ TRW }

define_leakage \
	-pinlist {C D E L Z2} \
	-vector {1 1 0 0 0} \
	-when "C*D*!E*!Z2" \
	{ TRW }

define_leakage \
	-pinlist {C D E L Z2} \
	-vector {1 1 1 0 1} \
	-when "C*D*E*Z2" \
	{ TRW }

define_arc \
	-type non_seq_setup \
	-pinlist {C D E L Z2} \
	-ic "0 $VDD 0 $VDD 0" \
	-vector {R F 0 X X} \
	-related_pin C \
	-pin D \
	-probe {L} \
	{ TRW }

define_arc \
	-type non_seq_hold \
	-pinlist {C D E L Z2} \
	-ic "0 $VDD 0 $VDD 0" \
	-vector {R F 0 X X} \
	-related_pin C \
	-pin D \
	-probe {L} \
	{ TRW }

define_arc \
	-type non_seq_setup \
	-pinlist {C D E L Z2} \
	-ic "0 0 0 0 0" \
	-vector {R R 0 X X} \
	-related_pin C \
	-pin D \
	-probe {L} \
	{ TRW }

define_arc \
	-type non_seq_hold \
	-pinlist {C D E L Z2} \
	-ic "0 0 0 0 0" \
	-vector {R R 0 X X} \
	-related_pin C \
	-pin D \
	-probe {L} \
	{ TRW }

define_arc \
	-type combinational \
	-pinlist {A Q_st Q} \
	-ic "0 $VDD 0" \
	-vector {R 1 R} \
	-related_pin A \
	-pin Q \
	{ COLL }

define_arc \
	-type combinational \
	-pinlist {A Q_st Q} \
	-ic "0 $VDD $VDD" \
	-vector {0 F F} \
	-related_pin Q_st \
	-pin Q \
	{ COLL }

define_arc \
	-type combinational \
	-pinlist {A Q_st Q} \
	-ic "$VDD 0 $VDD" \
	-vector {F 0 F} \
	-related_pin A \
	-pin Q \
	{ COLL }

define_arc \
	-type combinational \
	-pinlist {A Q_st Q} \
	-ic "$VDD 0 0" \
	-vector {1 R R} \
	-related_pin Q_st \
	-pin Q \
	{ COLL }

define_arc \
	-type hidden \
	-pinlist {A Q_st Q} \
	-ic "0 0 0" \
	-vector {R 0 0} \
	-pin A \
	{ COLL }

define_arc \
	-type hidden \
	-pinlist {A Q_st Q} \
	-ic "0 0 0" \
	-vector {0 R 0} \
	-pin Q_st \
	{ COLL }

define_arc \
	-type hidden \
	-pinlist {A Q_st Q} \
	-ic "$VDD $VDD $VDD" \
	-vector {F 1 1} \
	-pin A \
	{ COLL }

define_arc \
	-type hidden \
	-pinlist {A Q_st Q} \
	-ic "$VDD $VDD $VDD" \
	-vector {1 F 1} \
	-pin Q_st \
	{ COLL }

define_leakage -when "!A*!Q*!Q_st" { COLL }

define_leakage -when "A*Q*Q_st" { COLL }

define_leakage \
	-pinlist {A Q_st Q} \
	-vector {0 1 0} \
	-when "!A*!Q*Q_st" \
	{ COLL }

define_leakage \
	-pinlist {A Q_st Q} \
	-vector {0 1 1} \
	-when "!A*Q*Q_st" \
	{ COLL }

define_leakage \
	-pinlist {A Q_st Q} \
	-vector {1 0 1} \
	-when "A*Q*!Q_st" \
	{ COLL }

define_leakage \
	-pinlist {A Q_st Q} \
	-vector {1 0 0} \
	-when "A*!Q*!Q_st" \
	{ COLL }

define_arc \
	-type non_seq_setup \
	-pinlist {A Q_st Q} \
	-ic "$VDD 0 $VDD" \
	-vector {F R X} \
	-related_pin A \
	-pin Q_st \
	-probe {Q} \
	{ COLL }

define_arc \
	-type non_seq_hold \
	-pinlist {A Q_st Q} \
	-ic "$VDD 0 $VDD" \
	-vector {F R X} \
	-related_pin A \
	-pin Q_st \
	-probe {Q} \
	{ COLL }

define_arc \
	-type non_seq_setup \
	-pinlist {A Q_st Q} \
	-ic "0 $VDD 0" \
	-vector {R F X} \
	-related_pin A \
	-pin Q_st \
	-probe {Q} \
	{ COLL }

define_arc \
	-type non_seq_hold \
	-pinlist {A Q_st Q} \
	-ic "0 $VDD 0" \
	-vector {R F X} \
	-related_pin A \
	-pin Q_st \
	-probe {Q} \
	{ COLL }

define_arc \
	-type combinational \
	-pinlist {A B Q Qc Qn} \
	-ic "0 $VDD $VDD $VDD 0" \
	-vector {0 F F X X} \
	-related_pin B \
	-pin Q \
	{ C2P }

define_arc \
	-type combinational \
	-pinlist {A B Q Qc Qn} \
	-ic "0 $VDD $VDD $VDD 0" \
	-vector {0 F X F X} \
	-related_pin B \
	-pin Qc \
	{ C2P }

define_arc \
	-type combinational \
	-pinlist {A B Q Qc Qn} \
	-ic "0 $VDD $VDD $VDD 0" \
	-vector {0 F X X R} \
	-related_pin B \
	-pin Qn \
	{ C2P }

define_arc \
	-type combinational \
	-pinlist {A B Q Qc Qn} \
	-ic "$VDD 0 $VDD $VDD 0" \
	-vector {F 0 F X X} \
	-related_pin A \
	-pin Q \
	{ C2P }

define_arc \
	-type combinational \
	-pinlist {A B Q Qc Qn} \
	-ic "$VDD 0 $VDD $VDD 0" \
	-vector {F 0 X F X} \
	-related_pin A \
	-pin Qc \
	{ C2P }

define_arc \
	-type combinational \
	-pinlist {A B Q Qc Qn} \
	-ic "$VDD 0 $VDD $VDD 0" \
	-vector {F 0 X X R} \
	-related_pin A \
	-pin Qn \
	{ C2P }

define_arc \
	-type combinational \
	-pinlist {A B Q Qc Qn} \
	-ic "0 $VDD 0 0 $VDD" \
	-vector {R 1 R X X} \
	-related_pin A \
	-pin Q \
	{ C2P }

define_arc \
	-type combinational \
	-pinlist {A B Q Qc Qn} \
	-ic "0 $VDD 0 0 $VDD" \
	-vector {R 1 X R X} \
	-related_pin A \
	-pin Qc \
	{ C2P }

define_arc \
	-type combinational \
	-pinlist {A B Q Qc Qn} \
	-ic "0 $VDD 0 0 $VDD" \
	-vector {R 1 X X F} \
	-related_pin A \
	-pin Qn \
	{ C2P }

define_arc \
	-type combinational \
	-pinlist {A B Q Qc Qn} \
	-ic "$VDD 0 0 0 $VDD" \
	-vector {1 R R X X} \
	-related_pin B \
	-pin Q \
	{ C2P }

define_arc \
	-type combinational \
	-pinlist {A B Q Qc Qn} \
	-ic "$VDD 0 0 0 $VDD" \
	-vector {1 R X R X} \
	-related_pin B \
	-pin Qc \
	{ C2P }

define_arc \
	-type combinational \
	-pinlist {A B Q Qc Qn} \
	-ic "$VDD 0 0 0 $VDD" \
	-vector {1 R X X F} \
	-related_pin B \
	-pin Qn \
	{ C2P }

define_arc \
	-type hidden \
	-pinlist {A B Q Qc Qn} \
	-ic "0 0 0 0 $VDD" \
	-vector {R 0 0 0 1} \
	-pin A \
	{ C2P }

define_arc \
	-type hidden \
	-pinlist {A B Q Qc Qn} \
	-ic "0 0 0 0 $VDD" \
	-vector {0 R 0 0 1} \
	-pin B \
	{ C2P }

define_arc \
	-type hidden \
	-pinlist {A B Q Qc Qn} \
	-ic "$VDD $VDD $VDD $VDD 0" \
	-vector {F 1 1 1 0} \
	-pin A \
	{ C2P }

define_arc \
	-type hidden \
	-pinlist {A B Q Qc Qn} \
	-ic "$VDD $VDD $VDD $VDD 0" \
	-vector {1 F 1 1 0} \
	-pin B \
	{ C2P }

define_leakage -when "!A*!B*!Q*!Qc*Qn" { C2P }

define_leakage -when "A*B*Q*Qc*!Qn" { C2P }

define_leakage \
	-pinlist {A B Q Qc Qn} \
	-vector {0 1 1 1 0} \
	-when "!A*B*Q*Qc*!Qn" \
	{ C2P }

define_leakage \
	-pinlist {A B Q Qc Qn} \
	-vector {1 0 1 1 0} \
	-when "A*!B*Q*Qc*!Qn" \
	{ C2P }

define_leakage \
	-pinlist {A B Q Qc Qn} \
	-vector {0 1 0 0 1} \
	-when "!A*B*!Q*!Qc*Qn" \
	{ C2P }

define_leakage \
	-pinlist {A B Q Qc Qn} \
	-vector {1 0 0 0 1} \
	-when "A*!B*!Q*!Qc*Qn" \
	{ C2P }

define_arc \
	-type non_seq_setup \
	-pinlist {A B Q Qc Qn} \
	-ic "$VDD 0 $VDD $VDD 0" \
	-vector {F R X X X} \
	-related_pin A \
	-pin B \
	-probe {Q} \
	{ C2P }

define_arc \
	-type non_seq_hold \
	-pinlist {A B Q Qc Qn} \
	-ic "$VDD 0 $VDD $VDD 0" \
	-vector {F R X X X} \
	-related_pin A \
	-pin B \
	-probe {Q} \
	{ C2P }

define_arc \
	-type non_seq_setup \
	-pinlist {A B Q Qc Qn} \
	-ic "0 $VDD $VDD $VDD 0" \
	-vector {R F X X X} \
	-related_pin A \
	-pin B \
	-probe {Q} \
	{ C2P }

define_arc \
	-type non_seq_hold \
	-pinlist {A B Q Qc Qn} \
	-ic "0 $VDD $VDD $VDD 0" \
	-vector {R F X X X} \
	-related_pin A \
	-pin B \
	-probe {Q} \
	{ C2P }

define_arc \
	-type async \
	-pinlist {A B R Q} \
	-ic "$VDD $VDD 0 $VDD" \
	-vector {1 1 R F} \
	-related_pin R \
	-pin Q \
	{ RC2 }

define_arc \
	-type async \
	-pinlist {A B R Q} \
	-ic "$VDD $VDD $VDD 0" \
	-vector {1 1 F R} \
	-related_pin R \
	-pin Q \
	{ RC2 }

define_arc \
	-type combinational \
	-pinlist {A B R Q} \
	-ic "0 $VDD 0 $VDD" \
	-vector {0 F 0 F} \
	-related_pin B \
	-pin Q \
	{ RC2 }

define_arc \
	-type combinational \
	-pinlist {A B R Q} \
	-ic "$VDD 0 0 $VDD" \
	-vector {F 0 0 F} \
	-related_pin A \
	-pin Q \
	{ RC2 }

define_arc \
	-type combinational \
	-pinlist {A B R Q} \
	-ic "$VDD 0 0 0" \
	-vector {1 R 0 R} \
	-related_pin B \
	-pin Q \
	{ RC2 }

define_arc \
	-type combinational \
	-pinlist {A B R Q} \
	-ic "0 $VDD 0 0" \
	-vector {R 1 0 R} \
	-related_pin A \
	-pin Q \
	{ RC2 }

define_arc \
	-type hidden \
	-pinlist {A B R Q} \
	-ic "0 0 0 0" \
	-vector {R 0 0 0} \
	-pin A \
	{ RC2 }

define_arc \
	-type hidden \
	-pinlist {A B R Q} \
	-ic "0 0 0 0" \
	-vector {0 R 0 0} \
	-pin B \
	{ RC2 }

define_arc \
	-type hidden \
	-pinlist {A B R Q} \
	-ic "0 0 0 0" \
	-vector {0 0 R 0} \
	-pin R \
	{ RC2 }

define_arc \
	-type hidden \
	-pinlist {A B R Q} \
	-ic "0 0 $VDD 0" \
	-vector {0 0 F 0} \
	-pin R \
	{ RC2 }

define_arc \
	-type hidden \
	-pinlist {A B R Q} \
	-ic "0 $VDD $VDD 0" \
	-vector {0 F 1 0} \
	-pin B \
	{ RC2 }

define_arc \
	-type hidden \
	-pinlist {A B R Q} \
	-ic "$VDD 0 $VDD 0" \
	-vector {F 0 1 0} \
	-pin A \
	{ RC2 }

define_leakage -when "!A*!B*!Q*!R" { RC2 }

define_leakage -when "!A*!B*!Q*R" { RC2 }

define_leakage -when "!A*B*!Q*R" { RC2 }

define_leakage -when "A*!B*!Q*R" { RC2 }

define_leakage -when "A*B*Q*!R" { RC2 }

define_leakage -when "A*B*!Q*R" { RC2 }

define_leakage \
	-pinlist {A B R Q} \
	-vector {0 1 0 1} \
	-when "!A*B*Q*!R" \
	{ RC2 }

define_leakage \
	-pinlist {A B R Q} \
	-vector {1 0 0 1} \
	-when "A*!B*Q*!R" \
	{ RC2 }

define_leakage \
	-pinlist {A B R Q} \
	-vector {1 0 0 0} \
	-when "A*!B*!Q*!R" \
	{ RC2 }

define_leakage \
	-pinlist {A B R Q} \
	-vector {0 1 0 0} \
	-when "!A*B*!Q*!R" \
	{ RC2 }

define_arc \
	-type non_seq_setup \
	-pinlist {A B R Q} \
	-ic "$VDD 0 0 $VDD" \
	-vector {F R 0 X} \
	-related_pin A \
	-pin B \
	-probe {Q} \
	{ RC2 }

define_arc \
	-type non_seq_hold \
	-pinlist {A B R Q} \
	-ic "$VDD 0 0 $VDD" \
	-vector {F R 0 X} \
	-related_pin A \
	-pin B \
	-probe {Q} \
	{ RC2 }

define_arc \
	-type non_seq_setup \
	-pinlist {A B R Q} \
	-ic "$VDD $VDD $VDD 0" \
	-vector {F 1 F X} \
	-related_pin A \
	-pin R \
	-probe {Q} \
	{ RC2 }

define_arc \
	-type non_seq_hold \
	-pinlist {A B R Q} \
	-ic "$VDD $VDD $VDD 0" \
	-vector {F 1 F X} \
	-related_pin A \
	-pin R \
	-probe {Q} \
	{ RC2 }

define_arc \
	-type non_seq_setup \
	-pinlist {A B R Q} \
	-ic "0 $VDD 0 $VDD" \
	-vector {R F 0 X} \
	-related_pin A \
	-pin B \
	-probe {Q} \
	{ RC2 }

define_arc \
	-type non_seq_hold \
	-pinlist {A B R Q} \
	-ic "0 $VDD 0 $VDD" \
	-vector {R F 0 X} \
	-related_pin A \
	-pin B \
	-probe {Q} \
	{ RC2 }

define_arc \
	-type non_seq_setup \
	-pinlist {A B R Q} \
	-ic "$VDD $VDD $VDD 0" \
	-vector {1 F F X} \
	-related_pin B \
	-pin R \
	-probe {Q} \
	{ RC2 }

define_arc \
	-type non_seq_hold \
	-pinlist {A B R Q} \
	-ic "$VDD $VDD $VDD 0" \
	-vector {1 F F X} \
	-related_pin B \
	-pin R \
	-probe {Q} \
	{ RC2 }

