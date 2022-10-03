# Threshold funding contract

Inspired by https://en.wikipedia.org/wiki/Threshold_pledge_system and https://en.wikipedia.org/wiki/Assurance_contract.

Mechanism allowing for the conditional funding of a project. Funding goes through only if a minimum amount if met, otherwise each contributor is refunded.

Parameters: threshold, deadline, receiver address.

Deploy a new contract each time.

## Further ideas

- stake a precommitment to resolve prisoner's dilemna
  - I see that the other has staked towards cooperation, so now it's not rational for them to betray. They can see the same for me. We cooperate and maximize "total rationality"
- (already tackled by "kickstarting or threshold pledge system") actually fund project if the threshold of X coin is reached
- coordination, collective action, multilateral agreements
  - implement policy if a quorum of >N credibly commits
  - participate in manifestation, if >N persons commit to attend
  - let's switch to the other app, if majority is fof i
- a framework, contract factory, to facilitate this kind of thing
