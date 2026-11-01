// Balefang the Unslayable
// status: planned
// requires: a token that is a named card
// Flying, trample, protection from Angels and from Archons
// Players can't gain life.
// When Balefang enters, target opponent creates a tapped Baneslayer Angel token. The token is goaded for the rest of the game. (It's a {3}{W}{W} 5/5 Angel creature with flying, first strike, lifelink, and protection from Demons and from Dragons.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_4_0_rule(q: &dyn Query, asker: &Asker, player: PlayerId) -> bool { true }
#[allow(unused_variables)] fn ability_5_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Player(p) if (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&p)) }
static ABILITY_5_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_5_target_0)];
#[allow(unused_variables)] fn ability_5_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_5(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let created_0: Vec<ObjectId> = Vec::new();
planned!("a-token-that-is-a-named-card" => for player in { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.player()).into_iter().collect::<Vec<PlayerId>>() } { g.create_token_copies_tapped(Copiable::of(CardId::named("Baneslayer Angel")), player, { let q: &dyn Query = g; (1i64).max(0) as u32 }, true)?; });
g.end_sentence();
let objects_1 = { let q: &dyn Query = g; (created_0.clone()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let duration_1 = Duration::Indefinite; for object in objects_1 { cantrip_mechanics::goad::goad_for(g, item, object, duration_1); }
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Balefang the Unslayable", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 2, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Demon,Subtype::Dragon]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(5), toughness: Some(5), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying,Keyword::Trample]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_5_event), ability_5, "When Balefang enters, target opponent creates a tapped Baneslayer Angel token. The token is goaded for the rest of the game.").targets(ABILITY_5_TARGETS)], statics: &[cantrip_mechanics::protection::from_subtype::<{ cantrip_mechanics::lands::sub(Subtype::Angel) }>("Protection from Angel"),cantrip_mechanics::protection::from_subtype::<{ cantrip_mechanics::lands::sub(Subtype::Archon) }>("Protection from Archon")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, cant_gain_life: &[ability_4_0_rule], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
