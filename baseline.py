def histogram_256(nums: list[int]) -> list[int]:
    if not nums:
        return [0]*256
    mn = min(nums)
    mx = max(nums)
    hist = [0]*256
    if mx == mn:
        hist[0] = len(nums)
        return hist
    denom = mx - mn
    for x in nums:
        idx = ((x - mn) * 255) // denom
        # idx is guaranteed 0..255
        hist[idx] += 1
    return hist

if __name__ == "__main__":
    with open("histogram_1m.txt","r") as f:
        x = [int(y) for y in f.readlines()]
        hist = histogram_256(x)
        print(hist)