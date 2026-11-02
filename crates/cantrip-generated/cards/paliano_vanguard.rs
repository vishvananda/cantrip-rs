// Paliano Vanguard
// status: planned
// requires: ability DraftRule
// requires: object filter DraftNoted
// Draft this card face up.
// As you draft a creature card, you may reveal it, note its creature types, then turn this card face down.
// Other creatures you control of a type you noted for cards named Paliano Vanguard get +1/+1.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); (!(Some(asker.source) == Some(affected)) && ((*so_far).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(affected)) && planned_value!("object filter DraftNoted", false => q.draft_noted_card(affected, "DraftNoted(DraftNote { kind: CreatureType, player: AbilityController, card: \"Paliano Vanguard\" })")))) }
#[allow(unused_variables)] fn ability_2_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); let (p, t) = ((1i64) as i16, (1i64) as i16); chars.power = chars.power.map(|v| v.saturating_add(p)); chars.toughness = chars.toughness.map(|v| v.saturating_add(t)); }
planned_item!("ability-draft-rule" => draft_rules: cantrip_mechanics::draft::rule(&[DraftAction::DraftFaceUp]));
planned_item!("ability-draft-rule" => draft_rules: cantrip_mechanics::draft::rule(&[FaceUpDraftEffect::AsDraftACardOfType_MayRevealIt_NoteItsCreatureTypes_TurnThisDraftCardFaceDown(|q: &dyn Query, asker: &Asker, id: ObjectId| q.chars(id).is(CardType::Creature))]));

pub static CARD: CardDef = CardDef { name: "Paliano Vanguard", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Soldier]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[static_ability(Layer::PtModify, ability_2_affects, ability_2_apply_0, "Other creatures you control of a type you noted for cards named Paliano Vanguard get +1/+1.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
