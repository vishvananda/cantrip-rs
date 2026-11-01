// Drana, the Last Bloodchief
// status: ready
// Flying
// Whenever Drana attacks, defending player chooses a nonlegendary creature card in your graveyard. You return that card to the battlefield with an additional +1/+1 counter on it. The creature is a Vampire in addition to its other types.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Attacks { attacker, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(defending.0), u32::MAX]; if Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
static ENTERS_3: EntersAs = EntersAs { subtypes: &[Subtype::Vampire], ..EntersAs::NONE };
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let (carried_2_0,) = if let Some(present_2_0) = { let q: &dyn Query = g; Some(asker.source).and_then(|o: ObjectId| q.defending_player(o)) } { let selected_1_player = { let q: &dyn Query = g; present_2_0 }; let selected_1_candidates = { let q: &dyn Query = g; ((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::graveyard(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && (!(q.chars(id).supertypes.legendary) && q.chars(id).is(CardType::Creature))).collect::<Vec<ObjectId>>() }; let selected_1_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; let selected_1 = { cantrip_mechanics::choose::select(g, selected_1_player, asker.source, Purpose::PutOntoBattlefield, selected_1_candidates, cantrip_mechanics::select::Selection::of(selected_1_count, &[]))? };
(selected_1.clone(),) } else { Default::default() };
g.end_sentence();
let player_3 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let objects_3 = ({ let q: &dyn Query = g; (carried_2_0.clone()).get(0 as usize).copied().into_iter().collect::<Vec<ObjectId>>() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_3 = { let rows: Vec<(ObjectId, PlayerId)> = objects_3.iter().map(|&o| (o, player_3)).collect(); let counters: Vec<(CounterKind, i16)> = vec![(CounterKind::PlusOne, { let q: &dyn Query = g; (1i64).max(0) as i16 })]; g.put_all_onto_battlefield_becoming(&rows, item, &ENTERS_3, &counters)? };
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Drana, the Last Bloodchief", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 2, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Vampire,Subtype::Cleric]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(4), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ATTACKS.0), ability_1_event), ability_1, "Whenever Drana attacks, defending player chooses a nonlegendary creature card in your graveyard. You return that card to the battlefield with an additional +1/+1 counter on it. The creature is a Vampire in addition to its other types.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
