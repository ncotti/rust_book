use chapter_8_exercises::mean_mode;
use chapter_8_exercises::pig;
use chapter_8_exercises::company;

fn main() {
    mean_mode::mean_mode_main(&[0,1,2,3,4,5]);
    
    pig::pig_main("hello");
    pig::pig_main("tonto");
    pig::pig_main("aloha");
    pig::pig_main("al7ha");

    company::company_main();
}



