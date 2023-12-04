from datetime import date

def containsDuplicate(nums):
    for i,x in enumerate(nums):
        print("i,x",i,x)
        for y in nums[i+1:]:
            print("y",y)
            if x == y:return True

    return False



def topKFrequent(nums, k):
    """ this is a description"""
    hash_map = sorted({(k,nums.count(k)) for k in set(nums)},key=lambda x:x[1],reverse=True)
    print(hash_map)


topKFrequent([1,2,3,5,7,9,4,9,9,2],2)
