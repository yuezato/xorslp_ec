class Array
  def average_ratio
    100 * (self.sum / self.length)
  end
end

reg = /
[WithOUT comp.].*?MemAcc\ =\ (\d*),\ \#\[Fusioned\]MemAcc\ =\ (\d*).*?
[With comp.].*?MemAcc\ =\ (\d*).*?,\ \#\[Fusioned\]MemAcc\ =\ (\d*).*?
/xm

fname = ARGV[0]

File.open(fname) {|f|
  s = f.read
  r = s.scan(reg)

  coP_P = []
  fuP_P = []
  fuCoP_CoP = []
  fuCoP_P = []
  
  r.map {|array|
    p = array[0].to_f
    fuP = array[1].to_f
    coP = array[2].to_f
    fuCoP = array[3].to_f

    coP_P << coP / p
    fuP_P << fuP / p
    fuCoP_CoP << fuCoP / coP
    fuCoP_P << fuCoP / p
  }

  puts "Co(P)/P = #{coP_P.average_ratio} %"
  puts "Fu(P)/P = #{fuP_P.average_ratio} %"
  puts "Fu(Co(P))/Co(P) = #{fuCoP_CoP.average_ratio} %"
  puts "Fu(Co(P))/P = #{fuCoP_P.average_ratio} %"
}
