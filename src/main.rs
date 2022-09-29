use planet_defender::run;

fn main() {
    pollster::block_on(run());
}