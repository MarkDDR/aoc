use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    combinator::rest,
    multi::many1,
    sequence::{preceded, separated_pair, terminated},
    IResult, Parser,
};

fn take_line_rest(input: &str) -> IResult<&str, &str> {
    alt((take_until("\n"), rest)).parse(input)
}

#[derive(Debug, Clone)]
enum Meta {
    Size(u64),
    Dir(Vec<FsItem>),
}

impl Meta {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            tag("dir").map(|_| Self::Dir(vec![])),
            nom::character::complete::u64.map(|size| Self::Size(size)),
        ))
        .parse(input)
    }
}

#[derive(Debug, Clone)]
struct FsItem {
    name: String,
    meta: Meta,
}

impl FsItem {
    fn parse_line(input: &str) -> IResult<&str, Self> {
        separated_pair(Meta::parse, tag(" "), take_line_rest)
            .map(|(meta, name)| Self {
                name: name.into(),
                meta,
            })
            .parse(input)
    }
}

#[derive(Debug, Clone)]
enum Command {
    Ls,
    CdUp,
    CdTo(String),
}

impl Command {
    fn parse(input: &str) -> IResult<&str, Self> {
        preceded(
            tag("$ "),
            alt((
                tag("ls").map(|_| Self::Ls),
                tag("cd ..").map(|_| Self::CdUp),
                preceded(tag("cd "), take_line_rest).map(|place: &str| Self::CdTo(place.into())),
            )),
        )
        .parse(input)
    }
}

fn parse_header(input: &str) -> IResult<&str, FsItem> {
    tag("$ cd /\n")
        .map(|_| FsItem {
            name: "/".into(),
            meta: Meta::Dir(vec![]),
        })
        .parse(input)
}

fn parse_ls(input: &str) -> IResult<&str, Vec<FsItem>> {
    let (input, _) = tag("$ ls\n").parse(input)?;
    many1(terminated(FsItem::parse_line, tag("\n"))).parse(input)
}

fn build_file_tree(input: &str) -> IResult<&str, FsItem> {
    // let mut tree;
    let (input, mut tree) = parse_header(input)?;
    let mut cwd = &mut tree;

    while let Ok((input, command)) = Command::parse(input) {
        match command {
            Command::Ls => {
                
            },
            Command::CdUp => todo!(),
            Command::CdTo(_) => todo!(),
        }
    }

    todo!()
}

fn main() {
    let input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    let input = "$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
";
    println!("{:?}", parse_ls(input));
}
