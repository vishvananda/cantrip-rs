// Amy Pond
// status: ready
// Partner with Rory Williams (When this creature enters, target player may put Rory into their hand from their library, then shuffle.)
// Whenever Amy Pond deals combat damage to a player, choose a suspended card you own and remove that many time counters from it.
// Doctor's companion (You can have two commanders if the other is the Doctor.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Damage { source, target, amount, kind, .. } = event { let (source, amount, combat) = (*source, *amount, kind.is_combat()); let subject = Target::Object(source); let data = [u32::from(amount), match target { Target::Player(p) => u32::from(p.0), _ => u32::MAX }]; if combat && !q.state().obj(source).zone.is(ZoneKind::Stack) && Some(asker.source) == Some(source) && matches!(*target, Target::Player(p) if true) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut removed_0: i64 = 0;
let among_0_0 = { let q: &dyn Query = g; ((q.state().seats().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::exile(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && ((q.state().obj(id).zone.is(ZoneKind::Exile) && q.state().obj(id).counters(CounterKind::Time) > 0 && q.card(q.state().obj(id).card).suspend.is_some()) && Some(asker.controller) == Some(q.state().obj(id).owner))).collect::<Vec<ObjectId>>() }; let chooser_0_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let count_0_0 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; let objects_0_0 = cantrip_mechanics::choose::select(g, chooser_0_0, asker.source, Purpose::Choose, among_0_0, cantrip_mechanics::select::Selection::of(count_0_0, &[]))?;
let amount_0_0 = { let q: &dyn Query = g; (i64::from(item.data[0])).clamp(0, 32767) as i16 }; cantrip_api::instruction(g, |g| { for object in objects_0_0 { removed_0 += i64::from(g.remove_counters(object, CounterKind::Time, amount_0_0)); } });
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Amy Pond", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(2), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::DAMAGE.0), ability_1_event), ability_1, "Whenever Amy Pond deals combat damage to a player, choose a suspended card you own and remove that many time counters from it.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
