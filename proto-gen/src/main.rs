use pb_jelly_gen::GenProtos;

fn main() {
    GenProtos::builder()
        .out_path("../")
        .src_path("protos")
        .cleanup_out_path(false)
        .gen_protos();
}
