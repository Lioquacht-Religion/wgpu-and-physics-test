use wgpu_tutorial::test_game::TestGame;

fn main(){
    println!("hello");

    pollster::block_on(TestGame::run());

}
