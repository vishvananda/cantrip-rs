// Merfolk Spy
// status: ready
// Islandwalk (This creature can't be blocked as long as defending player controls an Island.)
// Whenever this creature deals combat damage to a player, that player reveals a card at random from their hand.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Damage { source, target, amount, kind, .. } = event { let (source, amount, combat) = (*source, *amount, kind.is_combat()); let subject = Target::Object(source); let data = [u32::from(amount), match target { Target::Player(p) => u32::from(p.0), _ => u32::MAX }]; if combat && !q.state().obj(source).zone.is(ZoneKind::Stack) && Some(asker.source) == Some(source) && matches!(*target, Target::Player(p) if true) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let player_0 = { let q: &dyn Query = g; Some(PlayerId(item.data[1] as u8)).expect("a reference that always names a player") }; let mut pool_0: Vec<ObjectId> = { let q: &dyn Query = g; q.state().zone(Zone::hand(player_0)).iter().copied().filter(|&id| true).collect::<Vec<ObjectId>>() };
let count_0 = { let q: &dyn Query = g; (1i64).max(0) as usize }; let mut shown_0: Vec<ObjectId> = Vec::new(); while shown_0.len() < count_0 && !pool_0.is_empty() { let at = g.choose_at_random(pool_0.len()); shown_0.push(pool_0.remove(at)); }
g.reveal(player_0, &shown_0);
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Merfolk Spy", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Merfolk,Subtype::Rogue]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(1), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Islandwalk]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::DAMAGE.0), ability_1_event), ability_1, "Whenever this creature deals combat damage to a player, that player reveals a card at random from their hand.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
