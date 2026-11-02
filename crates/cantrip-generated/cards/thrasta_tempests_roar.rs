// Thrasta, Tempest's Roar
// status: planned
// requires: engine: no TrampleOverPlaneswalkers
// This spell costs {3} less to cast for each other spell cast this turn.
// Trample, haste
// Trample over planeswalkers (This creature can deal excess combat damage to the controller of the planeswalker it's attacking.)
// Thrasta has hexproof as long as it entered this turn.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_cost(q: &dyn Query, source: ObjectId, player: PlayerId, costed: Costed, targets: &[Target]) -> CostDelta { let asker = &Asker::new(source, player); if costed == Costed::Spell(source) && true { CostDelta::less(((3i64 * q.history().this_turn().iter().filter_map(|r: &Record| -> Option<Fired> { let event = &r.event; if let Event::SpellCast { spell, player, .. } = event { let id = *spell; let subject = Target::Object(id); let data = [u32::from(player.0), u32::MAX]; if true && !(Some(asker.source) == Some(id)) { return Some(Fired { subject, data }); } }  None }).count() as i64)).clamp(0, 255) as u8) } else { CostDelta::NONE } }
#[allow(unused_variables)] fn ability_4_0_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); (Some(asker.source).is_some_and(|candidate: ObjectId| (q.history().entered().filter(|e| e.permanent == candidate).count() as i64) >= (1i64))) && (source == affected) }
#[allow(unused_variables)] fn ability_4_0_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); chars.keywords = chars.keywords.with(Keyword::Hexproof); }
planned_item!("engine-no-trample-over-planeswalkers" => keywords: Keywords::of(&[Keyword::TrampleOverPlaneswalkers]));

pub static CARD: CardDef = CardDef { name: "Thrasta, Tempest's Roar", token_words: &[], oracle: true, cost: ManaCost { generic: 10, x: 0, symbols: [0, 0, 0, 0, 2, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Dinosaur]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(7), toughness: Some(7), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Trample,Keyword::Haste]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[static_ability(Layer::Ability, ability_4_0_affects, ability_4_0_apply_0, "Thrasta has hexproof as long as it entered this turn.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, own_cost_changes: &[ability_0_cost], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
