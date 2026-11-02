// Endless Foot Assault
// status: ready
// Squad {1}{W} (As an additional cost to cast this spell, you may pay {1}{W} any number of times. When this enchantment enters, create that many tokens that are copies of it.)
// Whenever you attack, for each opponent, create a 1/1 black Ninja creature token that's tapped and attacking that player.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::AttackersDeclared { player, count } = event { let subject = Target::Player(*player); let data = [0u32; 2]; if *count > 0 && Some(asker.controller) == Some(*player) { return Some(Fired { subject, data }); } }
 None }
pub mod token_3 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Ninja).name, token_words: token!(Ninja).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Ninja]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(4)), power: Some(1), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
let mut gathered_1: Vec<(CardId, PlayerId, Target)> = Vec::new();
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { let def_2 = g.card_id(&token_3::CARD); let count_2 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let player_2 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let defender_2 = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") };
gathered_1.extend(std::iter::repeat((def_2, player_2, Target::Player(defender_2))).take(count_2 as usize));
    } 
let words_1 = g.words(asker.source); let mut created_1: Vec<ObjectId> = Vec::new();
let mut done_1: Vec<Target> = Vec::new(); for &(_, _, target) in &gathered_1 { if done_1.contains(&target) { continue; } done_1.push(target); let group: Vec<(CardId, PlayerId)> = gathered_1.iter().filter(|row| row.2 == target).map(|row| (row.0, row.1)).collect(); created_1.extend(g.create_tokens_worded(&group, true, Some(target), words_1)?); }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Endless Foot Assault", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[cantrip_mechanics::triggers::SQUAD,triggered(when_asked(EventMask(EventMask::ATTACKERS_DECLARED.0), ability_1_event), ability_1, "Whenever you attack, for each opponent, create a 1/1 black Ninja creature token that's tapped and attacking that player.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, kicker: multikicker(ManaCost { generic: 1, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_3::CARD];
