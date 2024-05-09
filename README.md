## Vision
The Study Group Smart Contract aims to facilitate the creation, management, and rewarding of study groups on a blockchain platform. The vision behind this project is to provide a decentralized solution for organizing study groups, allowing individuals to come together, collaborate on various topics, track contributions, and distribute rewards fairly based on contributions.

## Key Features
Group Creation: Users can create study groups by specifying the group name, topic, and the creator's address. This allows for the establishment of various study groups tailored to specific interests or subjects.

Membership Management: The contract manages membership within study groups. Users can join groups, and their contributions are tracked to ensure accountability and fair distribution of rewards.

Contribution Tracking: Members can record their contributions to the study group. This could include activities such as sharing study materials, leading discussions, or organizing events. The contract keeps track of each member's contribution score.

Reward Distribution: The contract enables the distribution of rewards among group members based on their contributions. Rewards are distributed proportionally to each member's contribution score, promoting fairness and incentivizing active participation.

## Implementation Details
The project is implemented using the Soroban SDK, a framework for developing smart contracts on blockchain platforms. It leverages Rust programming language for its reliability, performance, and safety features.

  StudyGroup Struct: Represents a study group with attributes such as name, topic, and creator's address.
  
  Membership Struct: Represents a membership within a study group, including attributes like group ID, member's address, and contribution score.
  
  StudyGroupContract: Implements the core functionalities of the study group smart contract, including group creation, membership management, contribution tracking, and reward distribution.

## How to Use
To interact with the Study Group Smart Contract:

  Deployment: Deploy the contract on a compatible blockchain platform.
  
  Group Creation: Call the create_group function to create a new study group.
  
  Membership: Users can join study groups by calling the join_group function.
  
  Contribution: Members can record their contributions using the record_contribution function.
  
  Reward Distribution: Call the distribute_rewards function to distribute rewards among group members.
