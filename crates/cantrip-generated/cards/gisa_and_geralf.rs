// Gisa and Geralf
// status: ready
// When Gisa and Geralf enters, mill four cards.
// Once during each of your turns, you may cast a Zombie creature spell from your graveyard.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (4i64).max(0) as u32 }; let mills_0: Vec<(PlayerId, u32)> = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }.into_iter().map(|p| (p, amount_0)).collect(); let milled_0 = g.mill_together(&mills_0);
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_0_cost(q: &dyn Query, asker: &Asker, card: ObjectId, player: PlayerId) -> Option<Cost> { (player == asker.controller && (q.chars(card).has_subtype(q.words(asker.source).subtype(Subtype::Zombie)) && q.chars(card).is(CardType::Creature)) && q.state().turn.active == player && !cantrip_mechanics::history::permission_used_this_turn(q, asker.source)).then(|| Cost::mana(q.card(q.chars(card).card).cost)) }

pub static CARD: CardDef = CardDef { name: "Gisa and Geralf", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 1, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Wizard]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(4), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_0_event), ability_0, "When Gisa and Geralf enters, mill four cards.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, granted_ways: &[GrantedWay { text: "Once during each of your turns, you may cast a Zombie creature spell from your graveyard.", permission: true, lands: false, cost: ability_1_0_cost, ..GrantedWay::CAST }], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
