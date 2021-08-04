class Array
  def average_ratio
    100 * (self.sum / self.length)
  end
end

reg1 = /
[WithOUT comp.].*?Variables\ =\ (\d*),\ \#\[Fusioned\]Variables\ =\ (\d*).*?
[With comp.].*?Variables\ =\ (\d*),\ \#\[Fusioned\]Variables\ =\ (\d*),\ \#\[Fusioned\&Scheduled\]Variables\ =\ (\d*)
/xm

reg2 = /
[WithOUT comp.].*?Capacity\ =\ (\d*),\ \#\[Fusioned\]Capacity\ =\ (\d*).*?
[With comp.].*?Capacity\ =\ (\d*),\ \#\[Fusioned\]Capacity\ =\ (\d*),\ \#\[Fusioned\&Scheduled\]Capacity\ =\ (\d*)
/xm

fname = ARGV[0]

def ratios(r)
  _CoP_P = []
  _FuP_P = []
  _FuCoP_CoP = []
  _DfsFuCoP_CoP = []
    
  r.map {|array|
    vals = {}
    vals[:P] = array[0].to_f
    vals[:FuP] = array[1].to_f
    vals[:CoP] = array[2].to_f
    vals[:FuCoP] = array[3].to_f
    vals[:DfsFuCoP] = array[4].to_f
      
    _CoP_P << (vals[:CoP] / vals[:P])
    _FuP_P << (vals[:FuP] / vals[:P])
    _FuCoP_CoP << (vals[:FuCoP] / vals[:CoP])
    _DfsFuCoP_CoP << (vals[:DfsFuCoP] / vals[:CoP])
  }

  return [_CoP_P.average_ratio, _FuP_P.average_ratio, _FuCoP_CoP.average_ratio, _DfsFuCoP_CoP.average_ratio]
end

File.open(fname) {|f|
  s = f.read

  _CoP_P, _FuP_P, _FuCoP_CoP, _DfsFuCoP_CoP = ratios(s.scan(reg1))
  
  puts "<NVar>"
  puts "  Co(P)/P = #{_CoP_P} %"
  puts "  Fu(P)/P = #{_FuP_P} %"
  puts "  Fu(Co(P))/Co(P) = #{_FuCoP_CoP} %"
  puts "  Dfs(Fu(Co(P)))/Co(P) = #{_DfsFuCoP_CoP} %"
  puts "</NVar>"

  _CoP_P, _FuP_P, _FuCoP_CoP, _DfsFuCoP_CoP = ratios(s.scan(reg2))

  puts ""
  puts "<CCap>"
  puts "  Co(P)/P = #{_CoP_P} %"
  puts "  Fu(P)/P = #{_FuP_P} %"
  puts "  Fu(Co(P))/Co(P) = #{_FuCoP_CoP} %"
  puts "  Dfs(Fu(Co(P)))/Co(P) = #{_DfsFuCoP_CoP} %"
  puts "</CCap>"
}
