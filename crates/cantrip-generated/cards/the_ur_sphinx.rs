// The Ur-Sphinx
// status: planned
// requires: cards played by All
// requires: source-zone scope on Static
// Eminence — As long as The Ur-Sphinx is in the command zone or on the battlefield, other Sphinx spells you cast cost {1} less to cast.
// Flying
// Whenever one or more Sphinxes you control attack, each player mills that many cards. For each player, you may cast a card that player milled this way without paying its mana cost.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_stated_0_rule(q: &dyn Query, source: ObjectId, player: PlayerId, costed: Costed, targets: &[Target]) -> CostDelta { let asker = &Asker::new(source, q.controller(source)); let Costed::Spell(spell) = costed else { return CostDelta::NONE; }; if player == asker.controller && (!(Some(asker.source) == Some(spell)) && q.chars(spell).has_subtype(q.words(asker.source).subtype(Subtype::Sphinx))) { CostDelta::less((1i64) as u8) } else { CostDelta::NONE } }
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Attacks { attacker, defender, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(q.controller(id).0), u32::from(defending.0)]; if (q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Sphinx)) && Some(asker.controller) == Some(q.controller(id))) && true { return Some(Fired { subject, data }); } }
 None }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (q.event_objects(item).len() as i64).max(0) as u32 }; let mills_0: Vec<(PlayerId, u32)> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() }.into_iter().map(|p| (p, amount_0)).collect(); let milled_0 = g.mill_together(&mills_0);
g.end_sentence();
let played_1: Vec<ObjectId> = planned_value!("cards played by All", Vec::<ObjectId>::new() => cantrip_mechanics::playing::play_each(g, { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() }, { let q: &dyn Query = g; (milled_0.clone()).into_iter().filter(|&id| true).collect::<Vec<ObjectId>>() }, CardPlayAction::CastSpell, PlayCount::UpTo({ let q: &dyn Query = g; (1i64).max(0) as u32 }), PlayCost::Free, false, false, asker.source)?);
g.end_sentence();
 Ok(()) }
planned_item!("source-zone-scope-on-static" => cost_changes: cantrip_mechanics::zones::scoped(&[EventZone::Command, EventZone::Battlefield], None, ability_0_stated_0_rule));

pub static CARD: CardDef = CardDef { name: "The Ur-Sphinx", token_words: &[], oracle: true, cost: ManaCost { generic: 6, x: 0, symbols: [1, 1, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Sphinx,Subtype::Avatar]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(10), toughness: Some(10), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ATTACKS.0), ability_2_event), ability_2, "Whenever one or more Sphinxes you control attack, each player mills that many cards. For each player, you may cast a card that player milled this way without paying its mana cost.").targets(&[]).once_per_batch()], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
