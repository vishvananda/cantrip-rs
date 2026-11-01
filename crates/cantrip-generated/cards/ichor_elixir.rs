// Ichor Elixir
// status: planned
// requires: replacement of RollPlanarDie
// If you would roll one or more planar dice, instead roll that many planar dice plus one and ignore one.
// {T}: Add {C}{C}.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_1_mana(g: &mut dyn Game, asker: &Asker, chosen: Option<Color>) -> Result<(),Suspend> { let mana_players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mana_0_0 = { let q: &dyn Query = g; (2i64).max(0) as u32 };
for player in mana_players_0 {
g.add_mana(player, Mana::C, mana_0_0);}
 Ok(()) }
planned_item!("replacement-of-roll-planar-die" => callbacks: #[allow(unused_variables)] fn ability_0_applies(q: &dyn Query, asker: &Asker, proposal: &Proposal) -> bool { if let Proposal::PlanarDieRoll(p) = proposal { let subject = Target::Player(p.player); let data = [u32::MAX; 2]; if Some(asker.controller) == Some(p.player) { return true; } }
 false }
#[allow(unused_variables, unused_mut)] fn ability_0_apply(q: &dyn Query, asker: &Asker, proposal: &mut Proposal) { let seen = *proposal; if let Proposal::PlanarDieRoll(seen) = &seen { let p = seen; let subject = Target::Player(p.player); let data = [u32::MAX; 2]; if let Proposal::PlanarDieRoll(p) = proposal { p.replaced = true;p.extra_dice += 1; p.ignore = Some(DiceIgnore::ChosenBy(subject.player().expect("a reference that always names a player"))); } } }
);
planned_item!("replacement-of-roll-planar-die" => replacements: replacement(ReplacementKind::PlanarDieRoll, ability_0_applies, ability_0_apply, "If you would roll one or more planar dice, instead roll that many planar dice plus one and ignore one."));

pub static CARD: CardDef = CardDef { name: "Ichor Elixir", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[mana_ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), Adds { pool: ManaPool([0, 0, 0, 0, 0, 2]), ..add_by(ability_1_mana) }, "{T}: Add {C}{C}.")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
