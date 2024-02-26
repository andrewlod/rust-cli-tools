use std::error::Error;
use std::{fs, io};

pub fn cat<S: AsRef<str>>(args: Vec<S>, mut out: impl io::Write) -> Result<(), Box<dyn Error>> {
    for path in args {
        let file_content = fs::read_to_string(path.as_ref())?;

        writeln!(&mut out, "{}", file_content)?;
    }

    Ok(())
}

macro_rules! cat {
    ($args:expr, $out:expr) => {
        cat($args, $out)
    };
    ($args:expr) => {
        cat::cat($args, io::stdout())
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cat() {
        let mut output = Vec::new();
        let args = vec!["assets/test_file.txt"];
        cat!(args, &mut output).expect("Command failed!");

        let output = String::from_utf8(output).expect("Failed to convert to UTF-8!");
        let parsed_output = output.replace('\r', "");
        let expected_value =
            "They were short staffed, so it took over an hour for her to get her burrito.
The stained glass window had slivers of yellow, orange, and red.
I couldn’t possibly love anything in the world more than I love my dog.
My mom made a milkshake with frozen bananas and chocolate sauce.
She thought Portuguese sangria was absolutely disgusting.
I imagine you think of this activity as something characters from a Jane Austin novel did.
You are bringing unresolved emotion into everything.
It was a real letdown that the baby had colic.
Tom is used to driving a pickup truck, but he's never driven a monster truck.
I don't think the house is as big as we hoped.
\"I don't appreciate your tone,\" the teacher said.
If you don't shut up, I will turn this car around.
He was sick of applying for jobs on job boards.
It is illegal to buy and sell tigers and other big cats in the United States.
Wearing long sleeves in the winter is a good idea.
";

        assert_eq!(parsed_output, expected_value);
    }
}
