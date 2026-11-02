// Niko Defies Destiny
// status: planned
// requires: mana spending purpose
// (As this Saga enters and after your draw step, add a lore counter. Sacrifice after III.)
// I — You gain 2 life for each foretold card you own in exile.
// II — Add {W}{U}. Spend this mana only to foretell cards or cast spells that have foretell.
// III — Return target card with foretell from your graveyard to your hand.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; ((2i64 * (((q.state().seats().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::exile(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && ({ let id = id; let o = q.state().obj(id); let own = AbilityRef::spell(o.card); q.card(o.card).foretell.is_some() && (q.state().permissions.iter().any(|p| p.by.object == id && p.by.ability == own) || q.history().cast_of(id).is_some_and(|cast| cast.granted_by().is_some_and(|by| by.ability == own))) } && Some(asker.controller) == Some(q.state().obj(id).owner))).collect::<Vec<ObjectId>>()).len() as i64)).min(i32::MAX.into()) as i32 }; let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_0 { g.gain_life(player, amount_0); }
g.end_sentence();
 Ok(()) }
planned_item!("mana-spending-purpose" => #[allow(dead_code, unused_variables)] fn mana_use_995(q: &dyn Query, asker: &Asker, spending: &Spending) -> bool { (spending.part == ManaPart::Foretell) || (spending.purpose == SpendingFor::Casting && spending.source.is_some_and(|spell| { let id = spell; let o = q.state().obj(id); let own = AbilityRef::spell(o.card); q.card(o.card).foretell.is_some() && (q.state().permissions.iter().any(|p| p.by.object == id && p.by.ability == own) || q.history().cast_of(id).is_some_and(|cast| cast.granted_by().is_some_and(|by| by.ability == own))) })) });
fn ability_0_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mana_players_2 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mana_2_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 };
let mana_2_1 = { let q: &dyn Query = g; (1i64).max(0) as u32 };
for player in mana_players_2 {
g.add_restricted_mana(player, Mana::W, mana_2_0, planned_value!("mana spending purpose", ManaRestriction::ABILITIES => ManaRestriction::Rule(&ManaRule { spend: mana_use_995, spent: None })));g.add_restricted_mana(player, Mana::U, mana_2_1, planned_value!("mana spending purpose", ManaRestriction::ABILITIES => ManaRestriction::Rule(&ManaRule { spend: mana_use_995, spent: None })));}
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_0_2_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if (q.state().obj(id).zone.kind() == ZoneKind::Graveyard) && !q.state().obj(id).token && ((q.card(q.chars(id).card).foretell.is_some() && (q.state().obj(id).zone.kind() == ZoneKind::Graveyard && Some(asker.controller) == Some(q.state().obj(id).owner))))) }
static ABILITY_0_2_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_2_target_0)];
fn ability_0_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let objects_3 = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>() }; let moved_3 = g.return_to_hand(&objects_3);
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Niko Defies Destiny", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [1, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[Subtype::Saga]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[chapter(1, ability_0_0, "I — You gain 2 life for each foretold card you own in exile.").targets(&[]),chapter(2, ability_0_1, "II — Add {W}{U}. Spend this mana only to foretell cards or cast spells that have foretell.").targets(&[]),chapter(3, ability_0_2, "III — Return target card with foretell from your graveyard to your hand.").targets(ABILITY_0_2_TARGETS)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
