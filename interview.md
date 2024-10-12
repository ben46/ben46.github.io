# 如何证明一个数据不存在一个默克尔树里面?

使用空值：在某些实现中，可以使用一个特殊的空值（如零哈希）来表示某个数据块的缺失。通过提供这个空值的路径，可以进一步简化证明过程。

# 算法题

如果一个函数, 输入的是一个按照时间排序的比特币价格的数组, 函数的目的是找到一个买入和卖出的下标, 来获取最大利润. 请你写一个线性时间复杂的算法

```py
def max_profit(prices):  
    if not prices:  
        return None, None  # 如果价格数组为空，返回 None  

    min_price = float('inf')  # 初始化最低价格为正无穷  
    max_profit = 0  # 初始化最大利润为 0  
    buy_index = 0  # 买入下标  
    sell_index = 0  # 卖出下标  

    for i, price in enumerate(prices):  
        # 如果当前价格低于最低价格，更新最低价格和买入下标  
        if price < min_price:  
            min_price = price  
            buy_index = i  
        
        # 计算当前利润  
        current_profit = price - min_price  
        
        # 如果当前利润高于最大利润，更新最大利润和卖出下标  
        if current_profit > max_profit:  
            max_profit = current_profit  
            sell_index = i  

    # 如果最大利润为 0，说明没有盈利机会  
    if max_profit > 0:  
        return buy_index, sell_index  
    else:  
        return None, None  # 如果没有盈利机会，返回 None  

# 示例用法  
prices = [7, 1, 5, 3, 6, 4]  
buy, sell = max_profit(prices)  
print(f"买入下标: {buy}, 卖出下标: {sell}")
```