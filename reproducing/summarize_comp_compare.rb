class Array
  def average_ratio
    100 * (self.sum / self.length)
  end
end

reg = /
\[NoComp\]\ \#XOR\ =\ (\d*).*?\#XOR\ =\ (\d*).*?\#XOR\ =\ (\d*)
/xm

fname = ARGV[0]

File.open(fname) {|f|
  s = f.read
  r = s.scan(reg)

  ratio_rep = []
  ratio_xor = []
  
  r.map {|array|
    no_comp = array[0].to_f
    repair  = array[1].to_f
    xor_rep = array[2].to_f

    ratio_rep << (repair / no_comp)
    ratio_xor << (xor_rep / no_comp)
  }

  puts "Repair(P)/P = #{ratio_rep.average_ratio} %"

  puts "XorRepair(P)/P = #{ratio_xor.average_ratio} %"
}
