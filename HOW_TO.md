# How to use

## Get ROC testnet tokens for the Rococo Contracts chain
Here is the [guide](https://use.ink/es/testnet/#2-get-testnet-tokens) to getting `ROC` tokens using the Rococo Faucet. 

## Using the contract
By following the instructions to upload & instantiate the smart contract (on the [README](README.md)), you will finish in the [Substrate - Contracts UI](https://contracts-ui.substrate.io/). 

At the instantiation, you can configure the maximum number of points a member can use to award other members, using the `max_points_per_sender` parameter. This can be modified in the future using the `set_max_points_per_sender` message.

The account used for the smart contract instantiation will be the `admin`. You can change it using the `set_admin` message:

### set_admin
![set_admin](https://user-images.githubusercontent.com/107150702/219525285-d64d2528-00eb-453f-8456-6ecb548fe2ea.png)
Only the current admin can change it. It only needs the new admin account as a parameter. 

> An important clarification: an admin is automatically a member of the organization.

To manage the members, the contract exposes the `add_member` & `remove_member` messages:

### add_member & remove_member
![add_member](https://user-images.githubusercontent.com/107150702/219526757-2a9a6fc8-6582-43e7-81e0-64eb8472fa1e.png)
![remove_member](https://user-images.githubusercontent.com/107150702/219526771-e3f9c366-f5b8-46ea-86d3-e61f097464be.png)
As with `set_admin`, only the current admin can add or remove members, and the corresponding account is the only one parameter. 

With the admin and members configured, the next step is to fund the contract. This can be done by transferring tokens with any account to the smart contract address. These tokens will be used as a reward at the distribution moment. 

Once the contract has funds, it's the awards time: 

### award
![award](https://user-images.githubusercontent.com/107150702/219528778-6042dcaf-8763-43b2-ac88-9d4145c01953.png)
The message just requires a recipient and the number of points. 

To award someone, the sender and the recipient must be members, and each sender has a maximum number of points to give (see `set_max_points_per_sender` message). 

At a certain point, the admin will declare finished the awards stage and start the rewards distribution:

### distribute_rewards
![distribute_rewards](https://user-images.githubusercontent.com/107150702/219529614-99311540-f781-4915-991a-9548f777ff2b.png)
Only the current admin can distribute the rewards, setting the `amount_to_distribuite` parameter or not. If the parameter is sent empty, the distribution will be made with the entire balance of the contract.  

Other useful messages:

### set_max_points_per_sender
Only the current admin can execute the message. 

It allows the modification of the maximum number of points a member can totally use to award other members. Only needs the new number of points sent as a parameter.  

### get_rewards_summary
This query message returns useful information about the awards and contract funds.

### get_max_points_per_sender
This query message returns the maximum number of points a member can totally use to award other members.

### get_sender_available_points
This query message returns the sender's available points to give to other members. 




