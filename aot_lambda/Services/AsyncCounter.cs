public class AsyncCounter {
    public async Task<string> run()
    {
        var result = "";

        await Task.Run(()=>
        {
            for (int i = 0; i < 10; i++)
            {
                result += "Method 2 : " + DateTime.Now.Millisecond + '\r'+'\n';
            }
        });
        
        return result;
    }
}
