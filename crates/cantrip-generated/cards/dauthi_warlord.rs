// Dauthi Warlord
// status: ready
// Shadow (This creature can block or be blocked by only creatures with shadow.)
// Dauthi Warlord's power is equal to the number of creatures on the battlefield with shadow.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); chars.power = Some(((q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && q.chars(id).keywords.has(Keyword::Shadow))).collect::<Vec<ObjectId>>()).len() as i64).clamp(-32768, 32767) as i16); }

pub static CARD: CardDef = CardDef { name: "Dauthi Warlord", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Dauthi,Subtype::Soldier]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(0), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Shadow]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[characteristic_defining(Layer::PtDefine, ability_1_apply_0, "Dauthi Warlord's power is equal to the number of creatures on the battlefield with shadow.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
