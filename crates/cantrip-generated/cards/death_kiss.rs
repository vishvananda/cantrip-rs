// Death Kiss
// status: planned
// requires: the X of the monstrosity a becomes-monstrous trigger reads
// Whenever a creature an opponent controls attacks one of your opponents, double its power until end of turn.
// {X}{X}{R}: Monstrosity X. (If this creature isn't monstrous, put X +1/+1 counters on it and it becomes monstrous.)
// When this creature becomes monstrous, goad up to X target creatures your opponents control.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Attacks { attacker, defender, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(q.controller(id).0), u32::from(defending.0)]; if (q.chars(id).is(CardType::Creature) && (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&q.controller(id))) && match *defender { Target::Player(p) => (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&p), _ => false } { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_0 = { let q: &dyn Query = g; (item.subject.object().into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_0 = { let q: &dyn Query = g; Some(asker.source) }; let duration_0 = Duration::EndOfTurn;
let kinds_0 = { let mut all = Vec::new(); for &object in &objects_0 { let kind = { let q: &dyn Query = g; EffectKind::Pump((Some(object).map_or(0i64, |o: ObjectId| i64::from(q.chars(o).power.unwrap_or(0)))) as i16, (0i64) as i16) }; all.push((object, kind));  } all }; if let Some(source) = source_0 { cantrip_api::instruction(g, |g| { for &(object, kind) in &kinds_0 { g.add_effect(source, object, kind, duration_0); } }); }
g.end_sentence();
 Ok(()) }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_1 = { let q: &dyn Query = g; (i64::from(item.x)).clamp(0, 32767) as i16 }; g.monstrosity(asker.source, amount_1);
g.end_sentence();
 Ok(()) }
planned_item!("the-x-of-the-monstrosity-a-becomes-monstrous-trigger-reads", "(kind:Triggered,args:(event:Object(event:BecomesMonstrous,objects:(kind:SameObject,args:(kind:Source))),intervening_if:None,limits:(per_turn:None,total:None),announcement:(targets:[(slot:(0),kind:Object((kind:Permanent,zones:[Battlefield])),count:UpTo((kind:AnnouncedX)),filter:(kind:All,args:[(kind:CardType,args:Creature),(kind:ControlledBy,args:(kind:OpponentsOf,args:(kind:AbilityController)))]),players:None,distribute:None,constraints:[],chooser:None)],modes:None),body:(instructions:[(kind:Effect,args:(result:(0),operation:(kind:KeywordEach,args:(actor:(kind:One,args:(kind:AbilityController)),action:Goad,objects:(kind:Targets,args:(0)),until:None))))])))");

pub static CARD: CardDef = CardDef { name: "Death Kiss", token_words: &[], oracle: true, cost: ManaCost { generic: 5, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Beholder]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(5), toughness: Some(5), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ATTACKS.0), ability_0_event), ability_0, "Whenever a creature an opponent controls attacks one of your opponents, double its power until end of turn.").targets(&[]),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 2, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), ability_1, "{X}{X}{R}: Monstrosity X.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
