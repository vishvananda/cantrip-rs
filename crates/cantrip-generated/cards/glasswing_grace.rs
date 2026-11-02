// Glasswing Grace // Age-Graced Chapel
// status: ready
// This land enters tapped.
// {T}: Add {W} or {B}.
// Enchant creature
// Enchanted creature gets +2/+2 and has flying and lifelink.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_enchant(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (q.chars(id).is(CardType::Creature))) }
#[allow(unused_variables)] fn ability_1_apply_1(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); chars.keywords = chars.keywords.with(Keyword::Flying);
chars.keywords = chars.keywords.with(Keyword::Lifelink); }
#[allow(unused_variables)] fn ability_1_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); let (p, t) = ((2i64) as i16, (2i64) as i16); chars.power = chars.power.map(|v| v.saturating_add(p)); chars.toughness = chars.toughness.map(|v| v.saturating_add(t)); }

pub static CARD: CardDef = CardDef { name: "Glasswing Grace", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [Some((Mana::W, Mana::B)),Some((Mana::W, Mana::B)),None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[Subtype::Aura]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[static_ability(Layer::Ability, cantrip_mechanics::attach::attached_object, ability_1_apply_1, "Enchanted creature gets +2/+2 and has flying and lifelink."),static_ability(Layer::PtModify, cantrip_mechanics::attach::attached_object, ability_1_apply_0, "Enchanted creature gets +2/+2 and has flying and lifelink.").continues()], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: Some(target(ability_0_enchant)), back: Some(&back::CARD), back_land: true, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
pub mod back {
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_0_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); entering.tapped = true;
 Ok(()) }
fn ability_1_mana(g: &mut dyn Game, asker: &Asker, chosen: Option<Color>) -> Result<(),Suspend> { let mana_players_1 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mana_1 = { let q: &dyn Query = g; (1i64).max(0) as u32 };
for player in mana_players_1 {
let color = chosen.expect("mana activation selected a color");
g.add_mana(player, color.mana(), mana_1);}
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Age-Graced Chapel", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Land]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[mana_ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), add_by(ability_1_mana).choose_color().colors(ColorSet(5)).yields_any(1), "{T}: Add {W} or {B}.")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_0_enters), modal: None, enchant: None, ..CardDef::BLANK };
}
