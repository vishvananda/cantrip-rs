// Arixmethes, Slumbering Isle
// status: ready
// Arixmethes enters tapped with five slumber counters on it.
// As long as Arixmethes has a slumber counter on it, it's a land. (It's not a creature.)
// Whenever you cast a spell, you may remove a slumber counter from Arixmethes.
// {T}: Add {G}{U}.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_0_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); entering.tapped = true;
let more = { let q: &dyn Query = g; (5i64).max(0) as i16 }; entering.counters[CounterKind::Slumber as usize] = entering.counters[CounterKind::Slumber as usize].saturating_add(more);
 Ok(()) }
#[allow(unused_variables)] fn ability_1_0_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); (Some(asker.source).is_some_and(|candidate: ObjectId| (i64::from(q.state().obj(candidate).counters(CounterKind::Slumber))) > (0i64))) && (source == affected) }
#[allow(unused_variables)] fn ability_1_0_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); let kept = TypeSet::of(&[CardType::Land]); chars.types = kept; chars.subtypes = chars.subtypes.iter().filter(|s| s.card_types().0 & kept.0 != 0).fold(SubtypeSet::NONE, SubtypeSet::with); }
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::SpellCast { spell, player, .. } = event { let id = *spell; let subject = Target::Object(id); let data = [u32::from(player.0), u32::MAX]; if Some(asker.controller) == Some(*player) && true { return Some(Fired { subject, data }); } }
 None }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let decider_2 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; if ({ let q: &dyn Query = g; Some(asker.source) }).is_some_and(|o| cantrip_mechanics::targets::permanent(g, o) && g.obj(o).counters(CounterKind::Slumber) > 0) && cantrip_mechanics::effects::ask_may(g, decider_2, asker.source, Purpose::Optional)? {
let mut removed_3: i64 = 0;
let objects_3_0 = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() };
let amount_3_0 = { let q: &dyn Query = g; (1i64).max(0) as i16 }; cantrip_api::instruction(g, |g| { for object in objects_3_0 { removed_3 += i64::from(g.remove_counters(object, CounterKind::Slumber, amount_3_0)); } });
g.end_sentence();
}
g.end_sentence();
 Ok(()) }
fn ability_3_mana(g: &mut dyn Game, asker: &Asker, chosen: Option<Color>) -> Result<(),Suspend> { let mana_players_4 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mana_4_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 };
let mana_4_1 = { let q: &dyn Query = g; (1i64).max(0) as u32 };
for player in mana_players_4 {
g.add_mana(player, Mana::U, mana_4_0);g.add_mana(player, Mana::G, mana_4_1);}
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Arixmethes, Slumbering Isle", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 1, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Kraken]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(12), toughness: Some(12), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::SPELL_CAST.0), ability_2_event), ability_2, "Whenever you cast a spell, you may remove a slumber counter from Arixmethes.").targets(&[]),mana_ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), Adds { pool: ManaPool([0, 1, 0, 0, 1, 0]), ..add_by(ability_3_mana) }, "{T}: Add {G}{U}.")], statics: &[static_ability(Layer::Type, ability_1_0_affects, ability_1_0_apply_0, "As long as Arixmethes has a slumber counter on it, it's a land.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_0_enters), modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
