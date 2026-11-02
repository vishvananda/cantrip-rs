// Lens of Clarity
// status: planned
// requires: cards a player may look at
// You may look at the top card of your library and at face-down creatures you don't control any time.
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("cards-a-player-may-look-at" => #[allow(unused_variables)] fn may_look_at_0(q: &dyn Query, asker: &Asker, player: PlayerId, card: ObjectId) -> bool { player == asker.controller && (q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && ({ let other = Some(asker.controller); q.state().apnap().filter(|p| Some(*p) != other).collect::<Vec<PlayerId>>() }).contains(&q.controller(id)))).collect::<Vec<ObjectId>>()).contains(&card) });
planned_item!("cards-a-player-may-look-at" => play_rules: PlayRule::MayLookAt(may_look_at_0));

pub static CARD: CardDef = CardDef { name: "Lens of Clarity", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, play_rules: &[PlayRule::LookAtLibraryTop], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
