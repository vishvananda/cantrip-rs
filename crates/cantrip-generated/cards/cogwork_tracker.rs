// Cogwork Tracker
// status: planned
// requires: ability DraftRule
// requires: condition NotedForDraft
// requires: static rule RequireCombat Attack
// Reveal this card as you draft it and note the player who passed it to you.
// This creature attacks each combat if able.
// This creature attacks a player you noted for cards named Cogwork Tracker each combat if able.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); chars.must_attack += 1; }
#[allow(unused_variables)] fn ability_2_0_rule(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { let source = asker.source; id == asker.source }
planned_item!("ability-draft-rule" => draft_rules: cantrip_mechanics::draft::rule(&[DraftAction::RevealThisDraftedCard, DraftAction::NotePlayerWhoPassedPackToYou]));
planned_item!("static-rule-require-combat-attack" => statics: object_rule(ability_2_0_rule, Rule::Must(Action::Attack(|q: &dyn Query, asker: &Asker, defender: Target| (match defender { Target::Player(p) => (q.state().apnap().filter(|&bound_0: &PlayerId| planned_value!("condition NotedForDraft", false => Some(bound_0).is_some_and(|p: PlayerId| q.draft_notes(p, DraftNoteKind::Player, "Cogwork Tracker")))).collect::<Vec<PlayerId>>()).contains(&p), Target::Object(id) => false && q.chars(id).is(CardType::Planeswalker) && (q.state().apnap().filter(|&bound_0: &PlayerId| planned_value!("condition NotedForDraft", false => Some(bound_0).is_some_and(|p: PlayerId| q.draft_notes(p, DraftNoteKind::Player, "Cogwork Tracker")))).collect::<Vec<PlayerId>>()).contains(&q.controller(id)), _ => false })), Required::AtLeast(1i64))));

pub static CARD: CardDef = CardDef { name: "Cogwork Tracker", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact,CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Dog,Subtype::Construct]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(4), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[static_ability(Layer::Ability, cantrip_mechanics::statics::itself, ability_1_apply_0, "This creature attacks each combat if able.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
