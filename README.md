This is the backend canister for managing user data for nofeebooking.


### How to test canister upgrades?
1. ensure there are not 'unstaged' files in git, 
2. checkout to the previous deployed tag 
3. deplyo that. 
4. checkout to new tag -> deploy that 


### deployment command
`git checkout v0.6.4`
`bash scripts/candid_generator.sh && dfx deploy`


### Before git tagging
1. ensure that the `bash scripts/candid_generator.sh` is run and the candid files are updated
2. ensure that did files are part of the commit. 

### push a specific git tag to git remote 
git push origin v0.6.4