use lazy_static::lazy_static;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Type {
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

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Effectiveness {
    Nothing,
    Half,
    Simple,
    Double,
}

impl Type {
    pub fn attack(&self, other: Type) -> Effectiveness {
        use Effectiveness::*;
        use Type::*;
        match self {
            Normal => match other {
                Rock | Steel => Half,
                Ghost => Nothing,
                _ => Simple,
            },
            Fight => match other {
                Normal | Rock | Steel | Dark | Ice => Double,
                Flying | Poison | Bug | Psychic | Fairy => Half,
                Ghost => Nothing,
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
                Steel => Nothing,
                _ => Simple,
            },
            Ground => match other {
                Poison | Rock | Steel | Fire | Electric => Double,
                Bug | Grass => Half,
                Flying => Nothing,
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
                Normal => Nothing,
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
                Ground => Nothing,
                _ => Simple,
            },
            Psychic => match other {
                Fight | Poison => Double,
                Steel | Psychic => Half,
                Dark => Nothing,
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
                Fairy => Nothing,
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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Pokemon {
    pub primary: Type,
    pub secondary: Option<Type>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Resistance {
    DoublyWeak,
    Weak,
    Standard,
    Resistant,
    DoublyResistant,
    Immune,
}

impl Pokemon {
    pub fn resist(&self, move_type: Type) -> Resistance {
        use Effectiveness::*;
        use Resistance::*;
        if let Some(secondary) = self.secondary {
            let ef_p = move_type.attack(self.primary);
            let ef_s = move_type.attack(secondary);
            match (ef_p, ef_s) {
                (Nothing, _) | (_, Nothing) => Immune,
                (Simple, Simple) | (Half, Double) | (Double, Half) => Standard,
                (Simple, Half) | (Half, Simple) => Resistant,
                (Simple, Double) | (Double, Simple) => Weak,
                (Half, Half) => DoublyResistant,
                (Double, Double) => DoublyWeak,
            }
        } else {
            match move_type.attack(self.primary) {
                Simple => Standard,
                Double => Weak,
                Half => Resistant,
                Nothing => Immune,
            }
        }
    }
}

lazy_static! {
    pub static ref ALL_TYPES: Vec<Type> = {
        use Type::*;
        vec![
            Normal, Fight, Flying, Poison, Ground, Rock, Bug, Ghost, Steel, Fire, Water, Grass,
            Electric, Psychic, Ice, Dragon, Dark, Fairy,
        ]
    };
    pub static ref ALL_POKEMON_NAIVE: Vec<Pokemon> = {
        let mut all = vec![];
        for single in &*ALL_TYPES {
            all.push(Pokemon {
                primary: *single,
                secondary: None,
            })
        }
        for primary in &*ALL_TYPES {
            for secondary in &*ALL_TYPES {
                if primary >= secondary {
                    continue;
                }
                all.push(Pokemon {
                    primary: *primary,
                    secondary: Some(*secondary),
                })
            }
        }
        all
    };
}
