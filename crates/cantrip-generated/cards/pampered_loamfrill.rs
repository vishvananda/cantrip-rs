// Pampered Loamfrill
// status: planned
// requires: engine: no perpetual effects
// requires: operation Conjure
// Deathtouch
// Renew — {1}{G}, Exile this card from your graveyard: Conjure a duplicate of another target creature card in your graveyard onto the top of your library. The duplicate perpetually gets +1/+1 and gains deathtouch. Activate only as a sorcery.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if (q.state().obj(id).zone.kind() == ZoneKind::Graveyard) && !q.state().obj(id).token && ((!(Some(asker.source) == Some(id)) && q.chars(id).is(CardType::Creature) && (q.state().obj(id).zone.kind() == ZoneKind::Graveyard && Some(asker.controller) == Some(q.state().obj(id).owner))))) }
static ABILITY_1_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_1_target_0)];
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let duplicated_0 = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>() };
let count_0: u32 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let mut conjured_0: Vec<ObjectId> = Vec::new(); for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { conjured_0.extend(planned_value!("operation Conjure", Vec::<ObjectId>::new() => cantrip_mechanics::conjure::conjure(g, player, { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }, asker.source, Conjured::DuplicateOf(&duplicated_0), count_0, ConjureTo::Library(ConjureSpot::Top))?)); }
g.end_sentence();
planned!("engine-no-perpetual-effects" => let objects_2: Vec<ObjectId> = { let q: &dyn Query = g; (conjured_0.clone()).get(0 as usize).copied().into_iter().collect::<Vec<ObjectId>>() };
for &object in &objects_2 { let kind_2_0 = { let q: &dyn Query = g; cantrip_mechanics::perpetual::Perpetual::Effect(EffectKind::Pump((1i64) as i16, (1i64) as i16)) }; cantrip_mechanics::perpetual::apply(g, asker.source, object, kind_2_0); let kind_2_1 = { let q: &dyn Query = g; cantrip_mechanics::perpetual::Perpetual::Effect(EffectKind::AddKeywords(Keywords::of(&[Keyword::Deathtouch]))) }; cantrip_mechanics::perpetual::apply(g, asker.source, object, kind_2_1); });
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Pampered Loamfrill", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Lizard]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(1), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Deathtouch]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(EXILE_THIS), ability_1, "Renew — {1}{G}, Exile this card from your graveyard: Conjure a duplicate of another target creature card in your graveyard onto the top of your library. The duplicate perpetually gets +1/+1 and gains deathtouch. Activate only as a sorcery.").targets(ABILITY_1_TARGETS).sorcery_speed().from(ZoneKind::Graveyard)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
