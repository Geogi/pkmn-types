fn main() {
    println!("Hello, world!");
}

enum Type {
    Normal,
    Fight,
    Flying,
    Poison,
    Ground,
    Rock,
    Bug,
    Ghost,
    Steel,
    Fire,
    Water,
    Grass,
    Electric,
    Psychic,
    Ice,
    Dragon,
    Dark,
    Fairy,
}

enum Efficiency {
    Simple,
    Double,
    Half,
    None,
}

impl Type {
    fn all() -> Vec<Type> {
        use Type::*;
        vec![
            Normal, Fight, Flying, Poison, Ground, Rock, Bug, Ghost, Steel, Fire, Water, Grass,
            Electric, Psychic, Ice, Dragon, Dark, Fairy,
        ]
    }
    fn attacks(&self, other: Type) -> Efficiency {
        use Efficiency::*;
        use Type::*;
        match self {
            Normal => match other {
                Rock | Steel => Half,
                Ghost => None,
                _ => Simple,
            },
            Fight => match other {
                Normal | Rock | Steel | Dark | Ice => Double,
                Flying | Poison | Bug | Psychic | Fairy => Half,
                Ghost => None,
                _ => Simple,
            },
            Flying => match other {
                Fight | Bug | Grass => Double,
                Rock | Steel | Electric => Half,
                _ => Simple,
            },
            Poison => match other {
                Grass | Fairy => Double,
                Poison | Ground | Rock | Ghost => Half,
                Steel => None,
                _ => Simple,
            },
            Ground => match other {
                Poison | Rock | Steel | Fire | Electric => Double,
                Bug | Grass => Half,
                Flying => None,
                _ => Simple,
            },
            Rock => match other {
                Flying | Bug | Fire | Ice => Double,
                Fight | Ground | Steel => Half,
                _ => Simple,
            },
            Bug => match other {
                Grass | Psychic | Dark => Double,
                Fight | Flying | Poison | Ghost | Steel | Fire | Fairy => Half,
                _ => Simple,
            },
            Ghost => match other {
                Ghost | Psychic => Double,
                Dark => Half,
                Normal => None,
                _ => Simple,
            },
            Steel => match other {
                Rock | Ice | Fairy => Double,
                Steel | Water | Fire | Electric => Half,
                _ => Simple,
            },
            Fire => match other {
                Bug | Steel | Grass | Ice => Double,
                Rock | Fire | Water | Dragon => Half,
                _ => Simple,
            },
            Water => match other {
                Ground | Rock | Fire => Double,
                Water | Grass | Dragon => Half,
                _ => Simple,
            },
            Grass => match other {
                Ground | Rock | Water => Double,
                Flying | Poison | Bug | Steel | Fire | Grass | Dragon => Half,
                _ => Simple,
            },
            Electric => match other {
                Flying | Water => Double,
                Grass | Electric | Dragon => Half,
                Ground => None,
                _ => Simple,
            },
            Psychic => match other {
                Fight | Poison => Double,
                Steel | Psychic => Half,
                None => Dark,
                _ => Simple,
            },
            Ice => match other {
                Flying | Ground | Grass | Dragon => Double,
                Steel | Fire | Water | Ice => Half,
                _ => Simple,
            },
            Dragon => match other {
                Dragon => Double,
                Steel => Half,
                Fairy => None,
                _ => Simple,
            },
            Dark => match other {
                Ghost | Psychic => Double,
                Fight | Dark | Fairy => Half,
                _ => Simple,
            },
            Fairy => match other {
                Fight | Dragon | Dark => Double,
                Poison | Steel | Fire => Half,
                _ => Simple,
            },
        }
    }
}
