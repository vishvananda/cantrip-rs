// Emissary of Grudges
// status: planned
// requires: a choice of Player outside its body
// requires: a spell or ability on the stack read as an object
// requires: a value chosen in secret
// requires: choose new targets for an ability
// requires: cost part RevealChoice
// Flying, haste
// As this creature enters, secretly choose an opponent.
// Reveal the player you chose: Choose new targets for target spell or ability if it's controlled by the chosen player and if it targets you or a permanent you control. Activate only once.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_2_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); let chosen_0 = planned_value!("a value chosen in secret", asker.controller => cantrip_mechanics::choose::value(g, { let q: &dyn Query = g; Target::Object(asker.source).object().map(|o: ObjectId| q.controller(o)).into_iter().collect::<Vec<PlayerId>>() }, asker.source, ChoiceKind::Player, ChoiceTerms { options: &[{ let q: &dyn Query = g; Target::Object(asker.source).object().map(|o: ObjectId| q.controller(o)).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() }], except: &[], count: None, secret: true, unchosen: false, random: false, ordered: false, up_to: false })?);
 Ok(()) }
#[allow(unused_variables)] fn ability_3_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { q.state().stack.iter().any(|e| (match t { Target::Object(id) => e.kind == StackKind::Spell && e.object == id, Target::Ability(serial) => e.kind != StackKind::Spell && e.serial == serial, _ => false }) && (true)) }
static ABILITY_3_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_3_target_0)];
fn ability_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
if { let q: &dyn Query = g; planned_value!("a spell or ability on the stack read as an object", None::<ObjectId> => q.stack_item_object(t_0_raw.get(0).copied())).is_some_and(|candidate: ObjectId| (planned_value!("a choice of Player outside its body", None::<PlayerId> => q.state().chosen_player(q.linked(item))) == Some(q.controller(candidate)) && q.state().stack.iter().find(|i| i.kind == StackKind::Spell && i.object == candidate).is_some_and(|i| q.state().targets(i).iter().any(|t| t.object().is_some_and(|o| (q.state().battlefield().iter().copied().filter(|&id| Some(asker.controller) == Some(q.controller(id))).collect::<Vec<ObjectId>>()).contains(&o)))))) } {
let chooser_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let retarget_1: Option<Target> = t_0_raw.get(0).copied().filter(|t| t_0.contains(t)); if let Some(t) = retarget_1 {
if let Target::Object(spell) = t { g.choose_new_targets(spell, chooser_1)?; }
else {
planned!("choose-new-targets-for-an-ability" => g.choose_new_targets_of(t, chooser_1)?);
}
}
g.end_sentence();
}
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Emissary of Grudges", token_words: &[], oracle: true, cost: ManaCost { generic: 5, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Efreet]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(6), toughness: Some(5), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying,Keyword::Haste]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(planned_value!("cost part RevealChoice", Cost::NONE => Cost::reveal_choice(ValueKind::Player))), ability_3, "Reveal the player you chose: Choose new targets for target spell or ability if it's controlled by the chosen player and if it targets you or a permanent you control. Activate only once.").targets(ABILITY_3_TARGETS).once()], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_2_enters), modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
