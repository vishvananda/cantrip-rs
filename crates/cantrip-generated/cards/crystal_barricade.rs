// Crystal Barricade
// status: ready
// Defender (This creature can't attack.)
// You have hexproof. (You can't be the target of spells or abilities your opponents control.)
// Prevent all noncombat damage that would be dealt to other creatures you control.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_0_rule(q: &dyn LayerQuery, source: ObjectId, player: PlayerId) -> bool { let asker = &Asker::new(source, q.controller(source)); player == asker.controller }
#[allow(unused_variables)] fn ability_2_applies(q: &dyn Query, asker: &Asker, e: &cantrip_api::DamageEvent) -> bool { { let (source, target, amount, combat) = (e.source, &e.target, e.amount, e.combat); let subject = *target; let data = [u32::from(amount), source.0]; if !combat && true && true && matches!(*target, Target::Object(o) if (!(Some(asker.source) == Some(o)) && (q.chars(o).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(o))))) { return true; } }
 false }

pub static CARD: CardDef = CardDef { name: "Crystal Barricade", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact,CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Wall]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(0), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Defender]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[PreventionDef { hint: "prevents all noncombat damage that would be dealt to other creatures you control", applies: ability_2_applies, amount: None }], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, player_keywords: &[PlayerKeywordRule { keyword: Keyword::Hexproof, applies: ability_1_0_rule }], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
