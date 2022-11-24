fn main() {
    let input = include_str!("../input.txt");
    let (paper, folds) = Paper::from(input);

    println!("Paper: {:?}", paper);
    println!("Folds: {:?}", folds);
    println!();

    println!("Original paper:");
    //paper.display();

    let paper_firstfold = paper.do_fold(folds[0].0, folds[0].1);

    println!("Visible dots after first fold: {}", paper.visible_dots());

    let paper_folded = paper.fold(&folds);
    println!();
    println!("Folded:");
    paper_folded.display();
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum FoldAxis {
    X,
    Y,
}

#[derive(Debug)]
struct Paper {
    dots: Vec<(usize, usize)>,
}

impl Paper {
    fn from(input: &str) -> (Self, Vec<(FoldAxis, usize)>) {
        let (dots, folds) = input.split_once("\n\n").unwrap();

        (
            Self {
                dots: dots
                    .lines()
                    .map(|line| {
                        let (x, y) = line.trim().split_once(",").unwrap();
                        (x.parse().unwrap(), y.parse().unwrap())
                    })
                    .collect(),
            },
            folds
                .lines()
                .map(|line| {
                    let (axis, position) = line[11..].split_once("=").unwrap();
                    (
                        match axis {
                            "x" => FoldAxis::X,
                            "y" => FoldAxis::Y,
                            _ => unreachable!(),
                        },
                        position.parse().unwrap(),
                    )
                })
                .collect(),
        )
    }

    fn size(&self) -> (usize, usize) {
        let (w, h) = self
            .dots
            .iter()
            .fold((0, 0), |max, dot| (dot.0.max(max.0), dot.1.max(max.1)));
        (w + 1, h + 1)
    }

    fn display(&self) {
        let (w, h) = self.size();

        for y in 0..h {
            for x in 0..w {
                if self.dots.contains(&(x, y)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    fn do_fold(&self, axis: FoldAxis, position: usize) -> Self {
        let mut newdots = self
            .dots
            .clone()
            .into_iter()
            .filter(|(dotx, doty)| {
                (axis == FoldAxis::X && *dotx < position)
                    || (axis == FoldAxis::Y && *doty < position)
            })
            .collect::<Vec<(usize, usize)>>();

        for (dotx, doty) in self.dots.iter() {
            let pos = match axis {
                FoldAxis::X => {
                    if *dotx > position {
                        let nx = position - (*dotx - position);
                        let ny = *doty;
                        Some((nx, ny))
                    } else {
                        None
                    }
                }
                FoldAxis::Y => {
                    if *doty > position {
                        let nx = *dotx;
                        let ny = position - (*doty - position);

                        Some((nx, ny))
                    } else {
                        None
                    }
                }
            };

            if let Some((x, y)) = pos {
                if !newdots.contains(&(x, y)) {
                    newdots.push((x, y));
                }
            }
        }

        Self { dots: newdots }
    }

    fn fold(self, folds: &[(FoldAxis, usize)]) -> Self {
        let mut newself = self;
        for (axis, position) in folds.iter() {
            newself = newself.do_fold(axis.clone(), *position)
        }

        newself
    }

    fn visible_dots(&self) -> usize {
        self.dots.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let input = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

        let (paper, folds) = Paper::from(input);

        println!("Paper: {:?}", paper);
        println!("Folds: {:?}", folds);
        println!();

        println!("Original paper:");
        paper.display();

        let paper_firstfold = paper.do_fold(folds[0].0, folds[0].1);

        println!();
        println!("After first fold: ");
        paper_firstfold.display();

        assert_eq!(paper_firstfold.visible_dots(), 17);

        let folded = paper.fold(&folds);
        println!();
        println!("After folding: ");
        folded.display();
    }
}
