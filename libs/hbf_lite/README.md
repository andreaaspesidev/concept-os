# HBF Lite
This `no-std` lite version of the `hbf_rs` library is made to be integrated into system components,
while minimizing the code size. To this purpose, most of the `syntactic sugar` of the original library
is sacrificed, and now it's no more required to store the whole HBF into a buffer in order to read it.