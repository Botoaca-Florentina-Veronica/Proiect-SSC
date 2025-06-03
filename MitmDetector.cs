using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;
using MongoDB.Bson;
using MongoDB.Driver;

public class MitmAlert
{
    public ObjectId Id { get; set; }
    public string Type { get; set; }
    public string Message { get; set; }
    public DateTime Timestamp { get; set; } = DateTime.UtcNow;
}

public class MitmDetector
{
    private readonly IMongoCollection<MitmAlert> _alerts;
    private readonly string _mongoConnectionString;
    private readonly string _dbName;

    public MitmDetector(string mongoConnectionString, string dbName)
    {
        _mongoConnectionString = mongoConnectionString;
        _dbName = dbName;
        var client = new MongoClient(mongoConnectionString);
        var db = client.GetDatabase(dbName);
        _alerts = db.GetCollection<MitmAlert>("alerts");
    }

    public List<MitmAlert> RunDetection()
    {
        var alerts = new List<MitmAlert>();
        alerts.AddRange(DetectArpSpoofing());
        alerts.AddRange(DetectDuplicateIp());
        return alerts;
    }

    private List<MitmAlert> DetectArpSpoofing()
    {
        var alerts = new List<MitmAlert>();
        var arpTable = GetArpTable();
        var macGroups = arpTable.GroupBy(e => e.IpAddress)
                                .Where(g => g.Select(x => x.MacAddress).Distinct().Count() > 1);

        foreach (var group in macGroups)
        {
            var msg = $"ARP anomaly: IP {group.Key} has multiple MACs: {string.Join(", ", group.Select(x => x.MacAddress))}";
            var alert = SaveAlert("ARP Spoofing", msg);
            alerts.Add(alert);
            Console.WriteLine(msg);
        }
        return alerts;
    }

    private List<MitmAlert> DetectDuplicateIp()
    {
        var alerts = new List<MitmAlert>();
        var arpTable = GetArpTable();
        var ipGroups = arpTable.GroupBy(e => e.MacAddress)
                               .Where(g => g.Select(x => x.IpAddress).Distinct().Count() > 1);

        foreach (var group in ipGroups)
        {
            var msg = $"Duplicate IP: MAC {group.Key} has multiple IPs: {string.Join(", ", group.Select(x => x.IpAddress))}";
            var alert = SaveAlert("Duplicate IP", msg);
            alerts.Add(alert);
            Console.WriteLine(msg);
        }
        return alerts;
    }

    private MitmAlert SaveAlert(string type, string message)
    {
        var alert = new MitmAlert { Type = type, Message = message };
        _alerts.InsertOne(alert);
        return alert;
    }

    private List<(string IpAddress, string MacAddress)> GetArpTable()
    {
        var result = new List<(string, string)>();
        var psi = new ProcessStartInfo("arp", "-a")
        {
            RedirectStandardOutput = true,
            UseShellExecute = false
        };
        using var proc = Process.Start(psi);
        string output = proc.StandardOutput.ReadToEnd();
        proc.WaitForExit();

        foreach (var line in output.Split('\n'))
        {
            var parts = line.Split(' ', StringSplitOptions.RemoveEmptyEntries);
            if (parts.Length >= 3 && parts[1].Contains("-"))
            {
                result.Add((parts[0], parts[1]));
            }
        }
        return result;
    }

    public List<MitmAlert> GetRecentAlerts(int count = 10)
    {
        return _alerts.Find(_ => true)
                     .Sort(Builders<MitmAlert>.Sort.Descending(x => x.Timestamp))
                     .Limit(count)
                     .ToList();
    }
}

class Program
{
    static void Main(string[] args)
    {
        if (args.Length < 2)
        {
            Console.WriteLine("Usage: MitmDetector.exe <mongo_connection_string> <database_name>");
            return;
        }

        string mongoConn = args[0];
        string dbName = args[1];
        
        try
        {
            var detector = new MitmDetector(mongoConn, dbName);
            var alerts = detector.RunDetection();
            
            Console.WriteLine($"Detected {alerts.Count} potential MITM attacks:");
            foreach (var alert in alerts)
            {
                Console.WriteLine($"[{alert.Timestamp}] {alert.Type}: {alert.Message}");
            }
        }
        catch (Exception ex)
        {
            Console.WriteLine($"Error: {ex.Message}");
        }
    }
}
