use wgpu_tutorial::tutorial1;

fn main() {
    pollster::block_on(tutorial1::run());
}
