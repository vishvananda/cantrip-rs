// Ardyn, the Usurper
// status: ready
// Demons you control have menace, lifelink, and haste.
// Starscourge — At the beginning of combat on your turn, exile up to one target creature card from a graveyard. If you exiled a card this way, create a token that's a copy of that card, except it's a 5/5 black Demon.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); ((*so_far).has_subtype(q.words(asker.source).subtype(Subtype::Demon)) && Some(asker.controller) == Some(q.controller(affected))) }
#[allow(unused_variables)] fn ability_0_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); chars.keywords = chars.keywords.with(Keyword::Menace);
chars.keywords = chars.keywords.with(Keyword::Lifelink);
chars.keywords = chars.keywords.with(Keyword::Haste); }
#[allow(unused_variables)] fn ability_1_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if (q.state().obj(id).zone.kind() == ZoneKind::Graveyard) && !q.state().obj(id).token && (q.chars(id).is(CardType::Creature))) }
static ABILITY_1_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_1_target_0).between(0, 1)];
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::BeginCombat) && Some(asker.controller) == Some(q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_2: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let objects_0 = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>() }; let moved_0 = g.exile_all(&objects_0);
g.end_sentence();
if { let q: &dyn Query = g; (((objects_0.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true).collect::<Vec<ObjectId>>()).len() as i64) > (0i64) } {
let original_1 = { let q: &dyn Query = g; (moved_0.clone()).get(0 as usize).copied() }; let values_1 = { let q: &dyn Query = g; original_1.map(|original: ObjectId| q.copiable_values(original).sized(5, 5).colored(ColorSet::NONE.with(q.words(asker.source).color(Color::Black))).typed(TypeSet::of(&[CardType::Creature]), SubtypeSet::of(&[q.words(asker.source).subtype(Subtype::Demon)]))) }; let count_1 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let player_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let created_1 = match values_1 { Some(values) => g.create_token_copies_tapped(values, player_1, count_1, false)?, None => Vec::new() };
g.end_sentence();
hoisted_2.extend(created_1.clone());
}
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Ardyn, the Usurper", token_words: &[], oracle: true, cost: ManaCost { generic: 5, x: 0, symbols: [0, 0, 3, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Elder,Subtype::Human,Subtype::Noble]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(4), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_1_event), ability_1, "Starscourge — At the beginning of combat on your turn, exile up to one target creature card from a graveyard. If you exiled a card this way, create a token that's a copy of that card, except it's a 5/5 black Demon.").targets(ABILITY_1_TARGETS)], statics: &[static_ability(Layer::Ability, ability_0_affects, ability_0_apply_0, "Demons you control have menace, lifelink, and haste.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
