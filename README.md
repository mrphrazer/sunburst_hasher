# sunburst_hasher

This tool is a performant and parallel hash collision finder for the [sunburst malware](https://github.com/fireeye/sunburst_countermeasures). It brute forces all strings in a defined range and checks if any of the resulting hashes matches a hash from the provided input file.



## Build

```
git clone https://github.com/mrphrazer/sunburst_hasher.git
cd sunburst_hasher
cargo build --release
```


## Run

```
cargo run --release -- --input <hash file> --min <minimum string length> --max <maximum string length>
```

## Example Run

```
$ cargo run --release -- --input examples/hashes.txt --min 1 --max 5

n = 1:



n = 2:
cb -- 5984963105389676759



n = 3:
gdb -- 10336842116636872171
idr -- 8129411991672431889
avp -- 13611051401579634621



n = 4:
fsma -- 3421213182954201407
fsni -- 3413886037471417852
egui -- 607197993339007484
epfw -- 17939405613729073960
fsfw -- 3407972863931386250
ksde -- 17633734304611248415
ppee -- 14710585101020280896
peid -- 9531326785919727076
date -- 16066522799090129502
dnsd -- 13316211011159594063
ekrn -- 3200333496547938354
cavp -- 17204844226884380288
idaq -- 14256853800858727521
fsms -- 3421197789791424393
fses -- 3413052607651207697
xagt -- 15695338751700748390



n = 5:
fsaua -- 12445177985737237804
scdbg -- 14868920869169964081
avgui -- 12709986806548166638
avpui -- 18147627057830191163
ehdrv -- 4931721628717906635
sense -- 16335643316870329598
close -- 14226582801651130532
ffdec -- 7412338704062093516
eelam -- 9559632696372799208
ilspy -- 10829648878147112121
dnspy -- 13825071784440082496
fsdfw -- 10393903804869831898
floss -- 18150909006539876521
fsbts -- 9333057603143916814
fsaus -- 12445232961318634374
```



