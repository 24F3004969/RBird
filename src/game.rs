mod position;
mod point;
mod tuple;

trait GameAssert {}
const SPLASH_SCREEN: &str = r#"
____________________.__           .___
\______   \______   \__|______  __| _/
 |       _/|    |  _/  \_  __ \/ __ |
 |    |   \|    |   \  ||  | \/ /_/ |
 |____|_  /|______  /__||__|  \____ |
        \/        \/               \/

    "#;
const B1: &str = r#"
                __     __------
            ___/o `\\ ,~   _~~  .
            ~ -.   ,'   _~-----
                `\\     ~~~--_'__
                  `~-==-~~~~~---'
            "#;
const B2: &str = r#"
                __
            ___/o `\\
            ~ -.   ,'\\~~~~~
                `\\    ~~~~--_'__
                  `~-==-~~~~~---'
             "#;
pub fn run() {
    println!("{:?}", position::Position::Center);
    println!("{}", SPLASH_SCREEN);
    println!("{}", B1);
    println!("{}", B2);
}

